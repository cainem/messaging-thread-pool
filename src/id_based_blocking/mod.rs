mod cloneable_id_based_writer;
pub mod id_based_writer;

use std::ffi::OsString;
use tracing::debug;
use tracing::subscriber::{self, DefaultGuard};
use tracing_core::LevelFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;

use self::{cloneable_id_based_writer::CloneableIdBasedWriter, id_based_writer::IdBasedWriter};

/// This struct is used to encapsulate the functionality of the IdBasedBlocking struct
///
/// The purpose of this struct is to encapsulate functionality to
/// output to a different (buffered) templated file name based on the pool item id.
///
/// It takes a base_filename as a parameter which it will use internally to base all trace file names on
/// If creates a tracing subscriber and holds on the the default guard to that subscriber, so the
/// subscriber will be dropped when the IdBasedBlocking struct is dropped
///
/// Internally it holds a copy of the last level set so that it can avoid unnecessary reloads.
///
/// The "blocking" in the name refers to the fact that the writes will block the calling thread until they have completed.
/// (although they are of course buffered)
///
/// The layer used with the writer is currently not configurable. This needs to be addressed at some point
#[derive(Debug)]
pub struct IdBasedBlocking {
    switcher: CloneableIdBasedWriter,
    // needs to be held to be kept alive
    _default_guard: DefaultGuard,
}

impl IdBasedBlocking {
    /// This function creates a new instance of the IdBasedBlocking struct
    pub fn new(base_filename: &str) -> Self {
        Self::new_with_targets(
            base_filename,
            Targets::new().with_default(LevelFilter::TRACE),
            IdBasedWriter::default_filename_formatter,
        )
    }

    pub fn new_with_targets(
        base_filename: &str,
        targets: Targets,
        filename_formatter: fn(&str, u64) -> OsString,
    ) -> Self {
        // Add trait bounds
        let id_based_writer =
            CloneableIdBasedWriter::new(IdBasedWriter::new(base_filename, filename_formatter));
        let cloned_id_based_writer = id_based_writer.clone();

        let layer = Layer::new();
        let subscriber =
            tracing_subscriber::registry().with(tracing_subscriber::Layer::with_filter(
                layer
                    .with_ansi(false)
                    .with_thread_ids(false)
                    .with_writer(move || cloned_id_based_writer.clone()),
                targets,
            ));

        // set-up tracing for this thread
        let default_guard = subscriber::set_default(subscriber);

        Self {
            switcher: id_based_writer,
            _default_guard: default_guard,
        }
    }

    pub fn set_id(&mut self, pool_item_id: u64) {
        self.switcher.switch(pool_item_id);
    }
}

impl Drop for IdBasedBlocking {
    fn drop(&mut self) {
        debug!("dropping IdBasedBlocking");
    }
}

#[cfg(test)]
mod tests {
    use const_format::concatcp;
    use std::fs;
    use tracing::{debug, info, warn};
    use tracing_core::LevelFilter;
    use tracing_subscriber::filter::Targets;

    use crate::{
        global_test_scope::test_scope,
        id_based_blocking::{id_based_writer::IdBasedWriter, IdBasedBlocking},
    };

    const TEST_DIR: &str = "target\\tmp";
    const TEST_PATH: &str = concatcp!(TEST_DIR, "\\switcher_test2");

    #[test]
    fn sanity_check() {
        let _ = fs::create_dir_all(TEST_DIR);

        let _ = fs::remove_file(IdBasedWriter::default_filename_formatter(TEST_PATH, 1));
        let _ = fs::remove_file(IdBasedWriter::default_filename_formatter(TEST_PATH, 2));

        test_scope(LevelFilter::INFO, || {
            info!("this should be logged to the console (1)");

            let mut target = IdBasedBlocking::new_with_targets(
                TEST_PATH,
                Targets::new().with_default(LevelFilter::INFO),
                IdBasedWriter::default_filename_formatter,
            );

            warn!("this warning should not be seen, no id set");

            target.set_id(1);

            info!("this info should be seen in 1");
            debug!("this debug should not be seen");

            target.set_id(2);
            info!("this info should be seen in 2");

            target.set_id(2);
            debug!("this info should not be seen in 2");

            target.set_id(1);

            drop(target);

            info!("this should be logged to the console (2)");
        });

        let result1 = fs::read_to_string(IdBasedWriter::default_filename_formatter(TEST_PATH, 1))
            .unwrap()
            .chars()
            .collect::<Vec<char>>();
        let result2 = fs::read_to_string(IdBasedWriter::default_filename_formatter(TEST_PATH, 2))
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        let expected_0 =
            "messaging_thread_pool::id_based_blocking::tests: this info should be seen in 1\n";
        assert_eq!(
            result1
                .into_iter()
                .rev()
                .take(expected_0.len())
                .rev()
                .collect::<String>(),
            expected_0
        );

        let expected_1 =
            "messaging_thread_pool::id_based_blocking::tests: this info should be seen in 2\n";
        assert_eq!(
            result2
                .into_iter()
                .rev()
                .take(expected_1.len())
                .rev()
                .collect::<String>(),
            expected_1
        );
    }
}
