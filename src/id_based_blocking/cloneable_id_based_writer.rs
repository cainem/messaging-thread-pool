use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use super::id_based_writer::IdBasedWriter;

/// The writer needs to be implement clone and be thread safe.
/// This is only intended to be used in a single threaded environment.
/// the mutex is only there to make it compile, it should not be used in
/// a multi-threaded environment and therefore should never block.
#[derive(Debug, Clone)]
pub struct CloneableIdBasedWriter {
    switcher: Arc<Mutex<IdBasedWriter>>,
}

impl CloneableIdBasedWriter {
    pub fn new(switcher: IdBasedWriter) -> Self {
        Self {
            switcher: Arc::new(Mutex::new(switcher)),
        }
    }

    pub fn switch(&self, pool_item_id: usize) {
        self.switcher
            .lock()
            .expect("only to be used in a single threaded environment")
            .set_pool_item(pool_item_id)
    }
}

impl Write for CloneableIdBasedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.switcher
            .lock()
            .expect("only to be used in a single threaded environment")
            .write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.switcher
            .lock()
            .expect("only to be used in a single threaded environment")
            .flush()
    }
}
