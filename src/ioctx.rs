use std::io::{self, Read, Write};
use std::default::Default;


pub trait IoCtx {
    fn read_input(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn write_input(&mut self, buf: &[u8]) -> io::Result<usize> {
        panic!("`write_input` unsupported for `StdIoCtx`");
    }
    fn write_output(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn read_output(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        panic!("`read_output` unsupported for `StdIoCtx`");
    }
    fn flush_output(&mut self) -> io::Result<()>;
}

// This workaround type wrapper for a generic T is directly in response to the error message:
// = note: only traits defined in the current crate can be implemented for a type parameter
// Solution taken from the error index: https://doc.rust-lang.org/error-index.html#E0210
// struct IoCtxType<T>(T);

impl Read for dyn IoCtx {
// impl<T> Read for IoCtxType<T> where T: IoCtx {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read_input(buf)
    }
}

impl Write for dyn IoCtx {
// impl<T> Write for IoCtxType<T> where T: IoCtx {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write_output(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush_output()
    }
}


/// Basic context using stdin and stdout impls for Read, Write
pub struct StdIoCtx {
    input: io::Stdin,
    output: io::Stdout,  // bf does not support stderr
}

impl Default for StdIoCtx {
    fn default() -> Self {
        Self {
            input: io::stdin(),
            output: io::stdout(),
        }
    }
}

impl IoCtx for StdIoCtx {
    fn read_input(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.input.read(buf) }
    fn write_output(&mut self, buf: &[u8]) -> io::Result<usize> { self.output.write(buf) }
    fn flush_output(&mut self) -> io::Result<()> { self.output.flush() }
}


/// Struct wrapper for u8 vector implementing Read, Write traits
struct ByteBuf {
    buf: Vec<u8>,
}

impl Default for ByteBuf {
    fn default() -> Self { Self { buf: Vec::new() } }
}

impl Read for ByteBuf {
    fn read(&mut self, input_buf: &mut [u8]) -> io::Result<usize> {
        // slice of input buffer for which Read is implemented
        let n_read = (&self.buf[..]).read(input_buf)?;
        self.buf.drain(..n_read);
        io::Result::Ok(n_read)
    }
}

impl Write for ByteBuf {
    fn write(&mut self, output_buf: &[u8]) -> io::Result<usize> { self.buf.write(output_buf) }
    fn flush(&mut self) -> io::Result<()> { self.buf.flush() }
}


/// IOContext supporting reading from input buffer, writing to output buffer, both of which
/// individually support both Read, Write or use in testing or other non-production environments
pub struct MockIoCtx {
    input: ByteBuf,
    output: ByteBuf,
}

impl Default for MockIoCtx {
    fn default() -> Self {
        Self {
            input: ByteBuf::default(),
            output: ByteBuf::default(),
        }
    }
}

impl IoCtx for MockIoCtx {
    fn read_input(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.input.read(buf) }
    fn write_input(&mut self, buf: &[u8]) -> io::Result<usize> { self.input.write(buf) }
    fn write_output(&mut self, buf: &[u8]) -> io::Result<usize> { self.output.write(buf) }
    fn read_output(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.output.read(buf) }
    fn flush_output(&mut self) -> io::Result<()> { self.output.flush() }
}
