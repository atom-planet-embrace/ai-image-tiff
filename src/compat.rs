//! Compatibility module providing BufReader and BufWriter for no_std environments.

use alloc::vec;
use alloc::vec::Vec;
use no_std_io::io::{self, BufRead, Read, Write};

/// A simple buffered reader for no_std environments.
pub struct BufReader<R> {
    inner: R,
    buf: Vec<u8>,
    pos: usize,
    filled: usize,
}

#[allow(dead_code)]
impl<R: Read> BufReader<R> {
    pub fn new(inner: R) -> Self {
        Self::with_capacity(8192, inner)
    }

    pub fn with_capacity(capacity: usize, inner: R) -> Self {
        BufReader {
            inner,
            buf: vec![0u8; capacity],
            pos: 0,
            filled: 0,
        }
    }
}

impl<R: Read> Read for BufReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // If we have buffered data, use it first
        let available = self.filled - self.pos;
        if available > 0 {
            let to_copy = buf.len().min(available);
            buf[..to_copy].copy_from_slice(&self.buf[self.pos..self.pos + to_copy]);
            self.pos += to_copy;
            return Ok(to_copy);
        }
        // If the request is larger than our buffer, read directly
        if buf.len() >= self.buf.len() {
            return self.inner.read(buf);
        }
        // Otherwise fill our buffer
        self.pos = 0;
        self.filled = self.inner.read(&mut self.buf)?;
        let to_copy = buf.len().min(self.filled);
        buf[..to_copy].copy_from_slice(&self.buf[..to_copy]);
        self.pos = to_copy;
        Ok(to_copy)
    }
}

impl<R: Read> BufRead for BufReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.pos >= self.filled {
            self.pos = 0;
            self.filled = self.inner.read(&mut self.buf)?;
        }
        Ok(&self.buf[self.pos..self.filled])
    }

    fn consume(&mut self, amt: usize) {
        self.pos = (self.pos + amt).min(self.filled);
    }
}

/// A simple buffered writer for no_std environments.
pub struct BufWriter<W> {
    inner: W,
    buf: Vec<u8>,
}

impl<W: Write> BufWriter<W> {
    pub fn new(inner: W) -> Self {
        BufWriter {
            inner,
            buf: Vec::with_capacity(8192),
        }
    }
}

impl<W: Write> Write for BufWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(buf);
        if self.buf.len() >= 8192 {
            self.inner.write_all(&self.buf)?;
            self.buf.clear();
        }
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.buf.extend_from_slice(buf);
        if self.buf.len() >= 8192 {
            self.inner.write_all(&self.buf)?;
            self.buf.clear();
        }
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        if !self.buf.is_empty() {
            self.inner.write_all(&self.buf)?;
            self.buf.clear();
        }
        self.inner.flush()
    }
}
