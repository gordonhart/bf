use std::io::{self, Read, Write};



/// Basic context using stdin and stdout impls for Read, Write
pub struct StdIOContext {
    input: io::Stdin,
    output: io::Stdout,  // bf does not support stderr
}

impl StdIOContext {
    fn new() -> Self {
        StdIOContext {
            input: io::stdin(),
            output: io::stdout(),
        }
    }
}

impl Read for StdIOContext {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.input.read(buf)
    }
}

impl Write for StdIOContext {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}



/// Struct wrapper for u8 vector implementing Read, Write traits
pub struct ByteBuf {
    buf: Vec<u8>,
}

impl Read for ByteBuf {
    fn read(&mut self, input_buf: &mut [u8]) -> io::Result<usize> {
        // slice of input buffer for which Read is implemented
        let n_read = (&self.buf[..]).read(input_buf)?;
        // we are not worried about underflow here because we trust the impl of Read used above
        self.buf.truncate(self.buf.len() - n_read);
        io::Result::Ok(n_read)
    }
}

impl Write for ByteBuf {
    fn write(&mut self, output_buf: &[u8]) -> io::Result<usize> {
        self.buf.write(output_buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf.flush()
    }
}



/// IOContext supporting reading from input buffer, writing to output buffer, both of which
/// individually support both Read, Write or use in testing or other non-production environments
pub struct MockIOContext {
    input: ByteBuf,
    output: ByteBuf,
}

impl MockIOContext {
    fn new() -> Self {
        MockIOContext {
            input: ByteBuf { buf: Vec::new() },
            output: ByteBuf { buf: Vec::new() },
        }
    }
}

impl Read for MockIOContext {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.input.read(buf)
    }
}

impl Write for MockIOContext {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}
