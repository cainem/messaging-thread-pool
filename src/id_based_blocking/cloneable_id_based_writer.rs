use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use super::id_based_writer::IdBasedWriter;

/// The writer needs to implement clone and be thread safe.
/// This is only intended to be used in a single threaded environment.
/// (as it will block badly in multi-threaded environments)
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
            .expect("no poisoned locks")
            .set_pool_item(pool_item_id)
    }
}

impl Write for CloneableIdBasedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.switcher.lock().expect("no poisoned lockes").write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.switcher.lock().expect("no poisoned locks").flush()
    }
}
