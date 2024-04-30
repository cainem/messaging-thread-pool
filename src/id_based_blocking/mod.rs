mod cloneable_id_based_writer;
mod id_based_writer;

use tracing::subscriber::{self, DefaultGuard};
use tracing_core::LevelFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::{
    filter,
    layer::SubscriberExt,
    reload::{self, Error, Handle},
    Registry,
};

use self::{cloneable_id_based_writer::CloneableIdBasedWriter, id_based_writer::IdBasedWriter};

#[derive(Debug)]
pub struct IdBasedBlocking {
    switcher: CloneableIdBasedWriter,
    reload_handle: Handle<LevelFilter, Registry>,
    last_level: Option<LevelFilter>,
    // needs to be held to be kept alive
    _default_guard: DefaultGuard,
}

impl IdBasedBlocking {
    /// This function creates a new instance of the IdBasedBlocking struct
    ///
    /// The purpose of this struct is to encapsulate functionality to
    /// a) to allow the tracing level to be changed via a filter reload handle
    /// b) output to a different (buffered) templated file name based on the pool item id.
    ///
    /// It takes a base_filename as a parameter which it will use internally to base all trace file names on
    /// If creates a tracing subscriber and holds on the the default guard to that subscriber, so the
    /// subscriber will be dropped when the IdBasedBlocking struct is dropped
    ///
    /// Internally it hold a reload handle which allows it to change the log level of the tracing subscriber if required.
    /// It holds a copy of the last level set so that it can avoid unnecessary reloads.
    pub fn new(base_filename: &str) -> Self {
        // Add trait bounds
        let id_based_writer = CloneableIdBasedWriter::new(IdBasedWriter::new(base_filename));
        let cloned_id_based_writer = id_based_writer.clone();
        let filter = filter::LevelFilter::OFF;
        let (filter, reload_handle) = reload::Layer::new(filter);

        let layer = Layer::new();
        let subscriber = tracing_subscriber::registry().with(filter).with(
            layer
                .with_ansi(false)
                .without_time()
                .with_writer(move || cloned_id_based_writer.clone()),
        );

        // set-up tracing for this thread
        let default_guard = subscriber::set_default(subscriber);

        Self {
            switcher: id_based_writer,
            reload_handle,
            last_level: None,
            _default_guard: default_guard,
        }
    }

    pub fn set_level_and_id(
        &mut self,
        level: LevelFilter,
        pool_item_id: usize,
    ) -> Result<(), Error> {
        self.switcher.switch(pool_item_id);

        if let Some(last_level) = self.last_level {
            if last_level == level {
                return Ok(());
            }
        }

        self.last_level = Some(level);
        self.reload_handle.reload(level)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tracing::{debug, info, warn};
    use tracing_core::LevelFilter;

    use crate::{
        global_test_scope::test_scope,
        id_based_blocking::{id_based_writer::IdBasedWriter, IdBasedBlocking},
    };

    const TEST_PATH: &str = "target\\tmp\\switcher_test2";

    #[test]
    fn sanity_check() {
        let _ = fs::remove_file(IdBasedWriter::filename_for(TEST_PATH, 1));
        let _ = fs::remove_file(IdBasedWriter::filename_for(TEST_PATH, 2));

        test_scope(LevelFilter::INFO, || {
            info!("this should be logged to the console (1)");

            let mut target = IdBasedBlocking::new(TEST_PATH);

            warn!("this warning should not be seen");

            target.set_level_and_id(LevelFilter::INFO, 1).unwrap();

            info!("this info should be seen in 1");
            debug!("this debug should not be seen");

            target.set_level_and_id(LevelFilter::INFO, 2).unwrap();
            info!("this info should be seen in 2");

            target.set_level_and_id(LevelFilter::OFF, 2).unwrap();
            info!("this info should not be seen in 2");

            target.set_level_and_id(LevelFilter::INFO, 1).unwrap();

            drop(target);

            info!("this should be logged to the console (2)");
        });

        let result1 = fs::read_to_string(IdBasedWriter::filename_for(TEST_PATH, 1)).unwrap();
        let result2 = fs::read_to_string(IdBasedWriter::filename_for(TEST_PATH, 2)).unwrap();

        assert_eq!(result1, " INFO messaging_thread_pool::id_based_blocking::tests: this info should be seen in 1\n");
        assert_eq!(result2, " INFO messaging_thread_pool::id_based_blocking::tests: this info should be seen in 2\n");
    }
}
