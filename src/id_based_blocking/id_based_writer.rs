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
pub struct IdBasedWriter {
    base_filename: String,
    last_set_pool_item_id: Option<usize>,
    last_written_pool_item_id: Option<usize>,
    writer_opt: Option<BufWriter<File>>,
    filename_formatter: fn(&str, usize) -> OsString,
}

impl IdBasedWriter {
    pub fn new<P>(base_filename: P, filename_formatter: fn(&str, usize) -> OsString) -> Self
    where
        P: AsRef<Path>,
    {
        IdBasedWriter {
            base_filename: base_filename.as_ref().to_string_lossy().to_string(),
            writer_opt: None,
            last_set_pool_item_id: None,
            last_written_pool_item_id: None,
            filename_formatter,
        }
    }

    pub fn set_pool_item(&mut self, pool_item_id: usize) {
        self.last_set_pool_item_id = Some(pool_item_id);
    }

    fn switch(&mut self, pool_item_id: usize) {
        let pool_item_id_opt = self.last_written_pool_item_id;
        match pool_item_id_opt {
            Some(t) if t == pool_item_id => {}
            _ => {
                // else we need to switch output files
                self.last_set_pool_item_id = Some(pool_item_id);
                self.close_old_open_new();
            }
        };
    }

    /// Determines the full filename based on the base filename and the pool item id.
    pub fn default_filename_formatter(base_filename: &str, pool_item_id: usize) -> OsString {
        OsString::from(format!("{}_{}.txt", base_filename, pool_item_id))
    }

    /// Opens a writer for the current file.
    fn close_old_open_new(&mut self) {
        if let Some(mut writer) = self.writer_opt.take() {
            // we have an existing writer we need to flush and close
            // before opening a new one
            // any failure is ignored here as there is very little that can be done
            let _ = writer.flush();
            drop(writer);
        }

        let log_file = (self.filename_formatter)(
            &self.base_filename,
            self.last_set_pool_item_id.expect("id to be set"),
        );
        match OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_file.clone())
        {
            Ok(file) => {
                self.writer_opt = Some(BufWriter::new(file));
            }
            Err(e) => panic!(
                "could not open file {} because of error: {}",
                log_file.to_string_lossy(),
                e
            ),
        };
    }
}

impl Write for IdBasedWriter {
    // forward writes and flushes to the internal writer
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.last_set_pool_item_id.is_none() {
            return Ok(0);
        }

        if self.last_written_pool_item_id != self.last_set_pool_item_id {
            self.switch(self.last_set_pool_item_id.expect("id to be set"));
            self.last_written_pool_item_id = self.last_set_pool_item_id;
        }

        // is last_write_thread_id == current_thread_id
        // yes just write
        // no flush and close current file if there is one; open a new one
        self.writer_opt
            .as_mut()
            .expect("writer to be set in order to get here")
            .write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.last_set_pool_item_id.is_none() {
            return Ok(());
        }

        self.writer_opt
            .as_mut()
            .expect("writer to be set in order to get here")
            .flush()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;

    use const_format::concatcp;

    use crate::id_based_blocking::id_based_writer::IdBasedWriter;

    const TEST_DIR: &str = "target\\tmp";
    const TEST_PATH: &str = concatcp!(TEST_DIR, "\\switcher_test");

    #[test]
    fn sanity_check() {
        let _ = fs::create_dir_all(TEST_DIR);

        let _ = fs::remove_file(IdBasedWriter::default_filename_formatter(TEST_PATH, 1));
        let _ = fs::remove_file(IdBasedWriter::default_filename_formatter(TEST_PATH, 2));

        let mut switcher = IdBasedWriter::new(TEST_PATH, IdBasedWriter::default_filename_formatter);

        switcher.set_pool_item(1);
        switcher.set_pool_item(1);
        switcher.write_all(b"test1").unwrap();
        switcher.set_pool_item(1);
        switcher.write_all(b"test1").unwrap();
        switcher.set_pool_item(2);
        switcher.write_all(b"test2").unwrap();

        drop(switcher);

        let result1 =
            fs::read_to_string(IdBasedWriter::default_filename_formatter(TEST_PATH, 1)).unwrap();
        let result2 =
            fs::read_to_string(IdBasedWriter::default_filename_formatter(TEST_PATH, 2)).unwrap();

        assert_eq!(result1, "test1test1");
        assert_eq!(result2, "test2");
    }
}
