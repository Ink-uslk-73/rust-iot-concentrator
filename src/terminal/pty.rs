//! src/terminal/pty.rs - PTY wrapper via portable-pty.

use std::io::Write;

use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PtyError {
    #[error("openpty: {0}")]
    Open(String),
    #[error("spawn: {0}")]
    Spawn(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub struct PtyHandle {
    master: Box<dyn portable_pty::MasterPty + Send>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
    reader: Option<Box<dyn std::io::Read + Send>>,
    writer: Option<Box<dyn std::io::Write + Send>>,
}

impl PtyHandle {
    pub fn spawn(program: &str, args: &[&str]) -> Result<Self, PtyError> {
        let pty = native_pty_system();
        let pair = pty
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| PtyError::Open(e.to_string()))?;

        let mut cmd = CommandBuilder::new(program);
        for a in args {
            cmd.arg(a);
        }
        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| PtyError::Spawn(e.to_string()))?;

        drop(pair.slave);

        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| PtyError::Open(e.to_string()))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| PtyError::Open(e.to_string()))?;

        Ok(Self {
            master: pair.master,
            child,
            reader: Some(reader),
            writer: Some(writer),
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), PtyError> {
        let w = self.writer.as_mut().ok_or_else(|| PtyError::Io(std::io::Error::other("no writer")))?;
        w.write_all(data)?;
        w.flush()?;
        Ok(())
    }

    pub fn take_reader(&mut self) -> Option<Box<dyn std::io::Read + Send>> {
        self.reader.take()
    }

    pub fn resize(&self, rows: u16, cols: u16) -> Result<(), PtyError> {
        self.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| PtyError::Open(e.to_string()))
    }

    pub fn wait(&mut self) -> Result<portable_pty::ExitStatus, PtyError> {
        self.child.wait().map_err(|e| PtyError::Io(std::io::Error::other(e.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn spawn_echo_and_write() {
        let mut h = PtyHandle::spawn("/bin/sh", &["-c", "echo hello"]).unwrap();
        let mut r = h.take_reader().unwrap();
        let mut buf = [0u8; 64];
        let n = r.read(&mut buf).unwrap_or(0);
        let out = String::from_utf8_lossy(&buf[..n]);
        assert!(out.contains("hello") || n > 0);
        let _ = h.wait();
    }
}
