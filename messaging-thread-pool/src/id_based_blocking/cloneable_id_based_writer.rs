use std::{
    cell::UnsafeCell,
    io::{self, Write},
    rc::Rc,
};

use super::id_based_writer::IdBasedWriter;

// Mark as Send and Sync
unsafe impl Send for CloneableIdBasedWriter {}
unsafe impl Sync for CloneableIdBasedWriter {}

#[derive(Debug, Clone)]
pub struct CloneableIdBasedWriter {
    writer: Rc<UnsafeCell<IdBasedWriter>>, // UnsafeCell for interior mutability
}

impl CloneableIdBasedWriter {
    pub fn new(writer: IdBasedWriter) -> Self {
        Self {
            writer: Rc::new(UnsafeCell::new(writer)),
        }
    }

    pub fn switch(&self, pool_item_id: u64) {
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
