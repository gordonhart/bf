use std::io::{self, Read, Write};
use std::default::Default;


pub trait IoCtx {
    fn read_input(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn write_input(&mut self, _: &[u8]) -> io::Result<usize> {
        panic!("`write_input` unsupported for `StdIoCtx`");
    }
    fn write_output(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn read_output(&mut self, _: &mut [u8]) -> io::Result<usize> {
        panic!("`read_output` unsupported for `StdIoCtx`");
    }
    fn flush_output(&mut self) -> io::Result<()>;
}

impl Read for dyn IoCtx {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.read_input(buf) }
}

impl Write for dyn IoCtx {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.write_output(buf) }
    fn flush(&mut self) -> io::Result<()> { self.flush_output() }
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


/// Same as StdIoCtx but flushes on every output
pub struct UnbufferedStdIoCtx { ctx: StdIoCtx }
impl Default for UnbufferedStdIoCtx {
    fn default() -> Self { Self { ctx: StdIoCtx::default() } }
}

impl IoCtx for UnbufferedStdIoCtx {
    fn read_input(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.ctx.input.read(buf) }
    fn write_output(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = self.ctx.output.write(buf)?;
        match self.flush_output() {
            Ok(_) => Ok(result),
            Err(e) => Err(e),
        }
    }
    fn flush_output(&mut self) -> io::Result<()> { self.ctx.output.flush() }
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
pub struct InMemoryIoCtx {
    input: ByteBuf,
    output: ByteBuf,
}

impl Default for InMemoryIoCtx {
    fn default() -> Self {
        Self {
            input: ByteBuf::default(),
            output: ByteBuf::default(),
        }
    }
}

impl IoCtx for InMemoryIoCtx {
    fn read_input(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.input.read(buf) }
    fn write_input(&mut self, buf: &[u8]) -> io::Result<usize> { self.input.write(buf) }
    fn write_output(&mut self, buf: &[u8]) -> io::Result<usize> { self.output.write(buf) }
    fn read_output(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.output.read(buf) }
    fn flush_output(&mut self) -> io::Result<()> { self.output.flush() }
}
