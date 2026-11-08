//! Fixtures shared by the crate's unit tests.

use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};

/// A fresh directory under the system temp dir, removed with everything in it
/// when dropped. Derefs to its path.
pub(crate) struct TempDir(PathBuf);

impl TempDir {
    pub(crate) fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!("lattice-{label}-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    /// Write `contents` to `relative`, creating its parent directories.
    pub(crate) fn write(&self, relative: &str, contents: impl AsRef<[u8]>) -> PathBuf {
        let path = self.0.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, contents).unwrap();
        path
    }
}

impl Deref for TempDir {
    type Target = Path;

    fn deref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for TempDir {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Create the same file or directory link on each host. Relative targets
/// are interpreted beside the link, as the OS interprets them.
pub(crate) fn symlink(original: impl AsRef<Path>, link: impl AsRef<Path>) -> std::io::Result<()> {
    let (original, link) = (original.as_ref(), link.as_ref());
    #[cfg(unix)]
    return std::os::unix::fs::symlink(original, link);
    #[cfg(windows)]
    {
        let target = link.parent().unwrap_or(Path::new(".")).join(original);
        if target.is_dir() {
            std::os::windows::fs::symlink_dir(original, link)
        } else {
            std::os::windows::fs::symlink_file(original, link)
        }
    }
}

/// An idle process owned by a lifecycle test, without relying on a Unix shell.
pub(crate) fn sleep_command(seconds: u64) -> std::process::Command {
    #[cfg(windows)]
    {
        let mut command = std::process::Command::new("powershell.exe");
        command.args(["-NoProfile", "-Command", &format!("Start-Sleep -Seconds {seconds}")]);
        command
    }
    #[cfg(unix)]
    {
        let mut command = std::process::Command::new("sleep");
        command.arg(seconds.to_string());
        command
    }
}
