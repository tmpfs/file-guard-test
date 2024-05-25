use anyhow::Result;
use file_guard::{try_lock, FileGuard, Lock};
use std::{
    fs::{File, OpenOptions},
    future::Future,
    io::ErrorKind,
    path::{Path, PathBuf},
    sync::Arc,
};

/// Exclusive file lock.
///
/// Used to prevent multiple applications from accessing
/// the same account simultaneously which could lead to
/// data corruption.
#[doc(hidden)]
pub struct FileLock {
    #[allow(dead_code)]
    guard: FileGuard<Arc<File>>,
    path: PathBuf,
}

impl FileLock {
    /// Try to acquire a file lock for a path.
    pub async fn acquire<F>(path: impl AsRef<Path>, on_message: impl Fn() -> F) -> Result<Self>
    where
        F: Future<Output = Result<()>>,
    {
        let file = Arc::new(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.as_ref())?,
        );

        let mut message_printed = false;

        loop {
            match try_lock(file.clone(), Lock::Exclusive, 0, 1) {
                Ok(guard) => {
                    println!("Lock acquired!");
                    return Ok(Self {
                        guard,
                        path: path.as_ref().to_path_buf(),
                    });
                }
                Err(e) => match e.kind() {
                    ErrorKind::WouldBlock => {
                        if !message_printed {
                            on_message().await?;
                            message_printed = true;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        continue;
                    }
                    _ => return Err(e.into()),
                },
            }
        }
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
