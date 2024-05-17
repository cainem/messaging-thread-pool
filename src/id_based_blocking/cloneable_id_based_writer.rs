use std::{
    cell::UnsafeCell,
    io::{self, Write},
    sync::Arc,
};

use super::id_based_writer::IdBasedWriter;

// Mark as Send and Sync
unsafe impl Send for CloneableIdBasedWriter {}
unsafe impl Sync for CloneableIdBasedWriter {}

#[derive(Debug, Clone)]
pub struct CloneableIdBasedWriter {
    writer: Arc<UnsafeCell<IdBasedWriter>>, // UnsafeCell for interior mutability
}

impl CloneableIdBasedWriter {
    pub fn new(writer: IdBasedWriter) -> Self {
        Self {
            writer: Arc::new(UnsafeCell::new(writer)),
        }
    }

    pub fn switch(&self, pool_item_id: usize) {
        let writer = unsafe { &mut *self.writer.get() };
        writer.set_pool_item(pool_item_id);
    }
}

impl Write for CloneableIdBasedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let writer = unsafe { &mut *self.writer.get() };
        writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let writer = unsafe { &mut *self.writer.get() };
        writer.flush()
    }
}
