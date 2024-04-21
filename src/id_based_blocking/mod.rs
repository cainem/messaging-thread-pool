mod cloneable_id_based_writer;
mod id_based_writer;

use tracing::subscriber::{self, DefaultGuard};
use tracing_core::LevelFilter;
use tracing_subscriber::{
    filter, fmt,
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
        let id_based_writer = CloneableIdBasedWriter::new(IdBasedWriter::new(base_filename));
        let cloned_id_based_writer = id_based_writer.clone();
        let filter = filter::LevelFilter::OFF;
        let (filter, reload_handle) = reload::Layer::new(filter);
        let subscriber = tracing_subscriber::registry()
            .with(filter)
            .with(fmt::Layer::new().with_writer(move || cloned_id_based_writer.clone()));

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
        if let Some(last_level) = self.last_level {
            if last_level == level {
                return Ok(());
            }
        }

        self.switcher.switch(pool_item_id);
        self.reload_handle.reload(level)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check_create_a() {
        todo!();
    }
}
