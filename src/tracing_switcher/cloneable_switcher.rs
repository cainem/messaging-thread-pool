use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use super::switcher::Switcher;

#[derive(Debug, Clone)]
pub struct CloneableSwitcher {
    switcher: Arc<Mutex<Switcher>>,
}

impl CloneableSwitcher {
    pub fn new(switcher: Switcher) -> Self {
        Self {
            switcher: Arc::new(Mutex::new(switcher)),
        }
    }

    pub fn switch(&self, pool_item_id: usize) {
        self.switcher.lock().unwrap().set_pool_item(pool_item_id)
    }
}

impl Write for CloneableSwitcher {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.switcher.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.switcher.lock().unwrap().flush()
    }
}
