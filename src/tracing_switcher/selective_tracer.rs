use tracing::subscriber::{self, DefaultGuard};
use tracing_core::LevelFilter;
use tracing_subscriber::{
    filter, fmt,
    layer::SubscriberExt,
    reload::{self, Error, Handle},
    Registry,
};

use super::{cloneable_switcher::CloneableSwitcher, switcher::Switcher};

#[derive(Debug)]
pub struct SwitcherHolder {
    switcher: CloneableSwitcher,
    reload_handle: Handle<LevelFilter, Registry>,
    _default_guard: DefaultGuard,
    last_level: Option<LevelFilter>,
}

impl SwitcherHolder {
    pub fn new(base_filename: &str) -> Self {
        let switcher = CloneableSwitcher::new(Switcher::new(base_filename));

        // TODO - I want this to be blocking!! should be easy right?
        let (non_blocking, guard) = tracing_appender::non_blocking(switcher.clone());

        let filter = filter::LevelFilter::OFF;

        let (filter, reload_handle) = reload::Layer::new(filter);

        let subscriber = tracing_subscriber::registry()
            .with(filter)
            .with(fmt::Layer::new().with_writer(non_blocking));
        // set-up tracing for this thread
        let default_guard = subscriber::set_default(subscriber);

        Self {
            switcher,
            reload_handle,
            _default_guard: default_guard,
            last_level: None,
        }
    }

    pub fn set_level(&mut self, level: LevelFilter, pool_item_id: usize) -> Result<(), Error> {
        if let Some(last_level) = self.last_level {
            if last_level == level {
                return Ok(());
            }
        }

        self.switcher.switch(pool_item_id);
        self.reload_handle.reload(level)
    }

    // pub fn set_level(&mut self, level: LevelFilter) {
    //     if let Some(last_level) = self.last_level {
    //         if last_level == level {
    //             return;
    //         }
    //     }

    //     self.last_level = Some(level);
    //     self.reload_handle.reload(level);
    // }

    // pub fn clone_switcher(&self) -> Arc<Mutex<Switcher>> {
    //     self.clone_switcher().clone()
    // }

    // pub fn reload_handle(&self) -> &Handle<LevelFilter, Registry> {
    //     &self.reload_handle
    // }
}
