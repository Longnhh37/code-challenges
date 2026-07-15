use std::cell::Cell;
use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    inner: R,
    bytes: Cell<usize>,
    reads: Cell<usize>,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            inner: wrapped,
            bytes: Cell::new(0),
            reads: Cell::new(0),
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes.get()
    }

    pub fn reads(&self) -> usize {
        self.reads.get()
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.inner.read(buf)?;

        self.reads.set(self.reads.get() + 1);
        self.bytes.set(self.bytes.get() + n);

        Ok(n)
    }
}

pub struct WriteStats<W> {
    inner: W,
    bytes: Cell<usize>,
    writes: Cell<usize>,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            inner: wrapped,
            bytes: Cell::new(0),
            writes: Cell::new(0),
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes.get()
    }

    pub fn writes(&self) -> usize {
        self.writes.get()
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.inner.write(buf)?;

        self.writes.set(self.writes.get() + 1);
        self.bytes.set(self.bytes.get() + n);

        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

