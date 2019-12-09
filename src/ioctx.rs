//! I/O context trait to read and write input and output streams of a running program.

use std::io::{self, Read, Write};
use std::default::Default;


/// Trait to read and write inputs and outputs to a BF program.
///
/// This trait is intended to provide a sufficiently general interface for program I/O for any use
/// case. The implementations defined here satisfy the needs of running a program in a tty, in a
/// test environment, as a library, etc.
///
/// Implementing a network-enabled `IoCtx` would, for example, open the door to writing a server in
/// BrainF\*ck...
pub trait IoCtx {
    /// Read bytes from the input stream.
    ///
    /// The implementation of `read_input` is used as the base `read` impl for the `Read` trait
    /// that is implemented for `dyn IoCtx`. This means that all provided functions of `Read`, e.g.
    /// `read_to_string`, operate on the input stream.
    fn read_input(&mut self, buf: &mut [u8]) -> io::Result<usize>;

    /// Write bytes to the output stream.
    ///
    /// The implementation of `write_output` is used as the base `write` impl for the `Write` trait
    /// that is implemented for `dyn IoCtx`. Provided methods of this trait, e.g. `write_all`, thus
    /// operate on the output stream.
    fn write_output(&mut self, buf: &[u8]) -> io::Result<usize>;

    /// Flush the output stream of the `IoCtx`. An implementations of `IoCtx` may or may not need
    /// to do anything when this is called.
    fn flush_output(&mut self) -> io::Result<()>;

    /// Read data from the output stream.
    ///
    /// # Panics
    ///
    /// The default implementation of `read_output` panics, as it it not necessary for an `IoCtx`
    /// to be able to read the output stream.
    fn read_output(&mut self, _: &mut [u8]) -> io::Result<usize> {
        panic!("`read_output` unsupported for `StdIoCtx`");
    }

    /// Write new data to the input stream.
    ///
    /// # Panics
    ///
    /// The default implementation panics as it is not necessary during program runtime for an
    /// `IoCtx` to be able to write to the input stream. This method may be implemented for an
    /// `IoCtx` that requires dynamic pre-loading of the program inputs before execution, e.g. in
    /// a test environment.
    fn write_input(&mut self, _: &[u8]) -> io::Result<usize> {
        panic!("`write_input` unsupported for `StdIoCtx`");
    }
}

impl Read for dyn IoCtx {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.read_input(buf) }
}

impl Write for dyn IoCtx {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.write_output(buf) }
    fn flush(&mut self) -> io::Result<()> { self.flush_output() }
}


/// Basic context using stdin and stdout implementations for `Read`, `Write`.
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

/// # Panics
///
/// Note that `write_input` and `read_output` are not implemented here, meaning the default
/// `panic!` implementation is used.
impl IoCtx for StdIoCtx {
    fn read_input(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.input.read(buf) }
    fn write_output(&mut self, buf: &[u8]) -> io::Result<usize> { self.output.write(buf) }
    fn flush_output(&mut self) -> io::Result<()> { self.output.flush() }
}


/// `StdIoCtx` that flushes the output stream on every call to `write_output`.
pub struct UnbufferedStdIoCtx { ctx: StdIoCtx }
impl Default for UnbufferedStdIoCtx {
    fn default() -> Self { Self { ctx: StdIoCtx::default() } }
}

/// # Panics
///
/// The default implementations for `write_input` and `read_output` are used here which panic
/// unconditionally.
impl IoCtx for UnbufferedStdIoCtx {
    fn read_input(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.ctx.input.read(buf) }
    fn write_output(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = self.ctx.output.write(buf)?;
        self.flush_output()?;
        Ok(result)
    }
    fn flush_output(&mut self) -> io::Result<()> { self.ctx.output.flush() }
}


/// Struct wrapper for u8 vector implementing `Read`, `Write` traits
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
        // empty the bytes that have just been read from the vector
        self.buf.drain(..n_read);
        io::Result::Ok(n_read)
    }
}

impl Write for ByteBuf {
    fn write(&mut self, output_buf: &[u8]) -> io::Result<usize> { self.buf.write(output_buf) }
    fn flush(&mut self) -> io::Result<()> { self.buf.flush() }
}


/// `IoCtx` supporting reading from input buffer, writing to output buffer, both of which
/// individually support both `Read`, `Write` for use when program output is intended to be
/// consumed by the process executing it, rather than a separate program or a human, both of which
/// are better served by the `StdIoCtx`.
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
