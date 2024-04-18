use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufWriter, Write};
use std::path::Path;

/// A `Switcher` is a writer that can switch between multiple output files.
/// it does it based on the current pool item id.
///
/// The filename is determined by the base filename and the pool item id.
/// Switching is done by calling `switch` with the current pool item id.
#[derive(Debug)]
pub struct Switcher {
    base_filename: String,
    pool_item_id: Option<usize>,
    writer_opt: Option<BufWriter<File>>,
}

impl Switcher {
    pub fn new<P>(base_filename: P) -> Self
    where
        P: AsRef<Path>,
    {
        Switcher {
            base_filename: base_filename.as_ref().to_string_lossy().to_string(),
            writer_opt: None,
            pool_item_id: None,
        }
    }

    pub fn set_pool_item(&mut self, pool_item_id: usize) {
        todo!();
        // todo set the intended item id
        // the actual item id will be set lazily on write
    }

    fn switch(&mut self, thread_id: usize) -> io::Result<()> {
        let pool_id_opt = self.pool_item_id;
        match pool_id_opt {
            Some(t) if t == thread_id => Ok(()),
            _ => {
                // else we need to switch output files
                self.pool_item_id = Some(thread_id);
                self.close_old_open_new()
            }
        }
    }

    /// Determines the full filename based on the base filename and the pool item id.
    pub fn filename_for(base_filename: &str, pool_item_id: usize) -> OsString {
        OsString::from(format!("{}.{}.ansi", base_filename, pool_item_id))
    }

    /// Opens a writer for the current file.
    fn close_old_open_new(&mut self) -> io::Result<()> {
        if let Some(mut writer) = self.writer_opt.take() {
            // we have an existing writer we need to flush and close
            // before opening a new one
            writer.flush()?;
            drop(writer);
        }

        let p = Self::filename_for(
            &self.base_filename,
            self.pool_item_id.expect("id to be set"),
        );
        let f = OpenOptions::new().append(true).create(true).open(&p)?;
        self.writer_opt = Some(BufWriter::new(f));
        Ok(())
    }
}

impl Write for Switcher {
    // forward writes and flushes to the internal writer
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // is last_write_thread_id == current_thread_id
        // yes just write
        // no flush and close current file if there is one; open a new one
        self.writer_opt
            .as_mut()
            .expect("writer to be set in order to get here")
            .write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer_opt
            .as_mut()
            .expect("writer to be set in order to get here")
            .flush()
    }
}

#[cfg(test)]
mod tests {
    use crate::tracing_switcher::switcher::Switcher;
    use std::fs;
    use std::io::Write;

    const TEST_PATH: &str = "target\\tmp\\switcher_test";

    #[test]
    fn sanity_check() {
        let _ = fs::remove_file(Switcher::filename_for(TEST_PATH, 1));
        let _ = fs::remove_file(Switcher::filename_for(TEST_PATH, 2));

        let mut switcher = Switcher::new(TEST_PATH);

        switcher.switch(1).unwrap();
        switcher.write_all(b"test1").unwrap();
        switcher.switch(2).unwrap();
        switcher.write_all(b"test2").unwrap();

        drop(switcher);

        let result1 = fs::read_to_string(Switcher::filename_for(TEST_PATH, 1)).unwrap();
        let result2 = fs::read_to_string(Switcher::filename_for(TEST_PATH, 2)).unwrap();

        assert_eq!(result1, "test1");
        assert_eq!(result2, "test2");
    }
}
