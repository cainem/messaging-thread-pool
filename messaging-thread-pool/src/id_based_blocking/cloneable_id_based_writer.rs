use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};
use tracing::error;

use super::id_based_writer::IdBasedWriter;

/// Cloneable writer wrapper used by tracing's `with_writer` callback.
///
/// Tracing obtains a writer by cloning this type, so interior coordination is
/// required to ensure each clone routes to the same underlying `IdBasedWriter`.
/// A mutex is used for correctness; this may add contention under very high
/// logging volume, but keeps the implementation memory-safe.
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
        match self.writer.lock() {
            Ok(mut writer) => writer.set_pool_item(pool_item_id),
            Err(err) => {
                error!(
                    "failed to lock id based writer in switch; id change skipped: {}",
                    err
                );
            }
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
