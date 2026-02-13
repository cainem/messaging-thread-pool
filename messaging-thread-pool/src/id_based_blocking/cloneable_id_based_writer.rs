use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use super::id_based_writer::IdBasedWriter;

#[derive(Debug, Clone)]
pub struct CloneableIdBasedWriter {
    writer: Arc<Mutex<IdBasedWriter>>,
}

impl CloneableIdBasedWriter {
    pub fn new(writer: IdBasedWriter) -> Self {
        Self {
            writer: Arc::new(Mutex::new(writer)),
        }
    }

    pub fn switch(&self, pool_item_id: u64) {
        if let Ok(mut writer) = self.writer.lock() {
            writer.set_pool_item(pool_item_id);
        }
    }
}

impl Write for CloneableIdBasedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| io::Error::other("failed to lock id based writer"))?;
        writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| io::Error::other("failed to lock id based writer"))?;
        writer.flush()
    }
}
