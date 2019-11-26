use std::fmt::Debug;
use std::io::{self, Read, Write};

/// Basic context using stdin and stdout impls for Read, Write
pub struct StdIOContext {
    input: io::Stdin,
    output: io::Stdout,  // bf does not support stderr
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

// TODO: figure out how to not duplicate this with StdIOContext's impl
impl Write for MockIOContext {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

fn test<T: Read + Write>(ctx: &mut T) {
    ctx.write(&b"123"[..]);
}

fn main() -> io::Result<()> {
    // let mut ctx = IOContext::using_stds();
    /*
    let o: Vec<u8> = Vec::new();
    let mut ctx = IOContext {
        input: Box::new(std::io::stdin()),
        output: Box::new(o),
        errput: Box::new(std::io::stderr()),
    };
    // let o: &Vec<u8> = &(*ctx.output);
    // println!("{:?}", ctx.input.read_to_end()?);
    test(&mut ctx);
    let o2: Vec<u8> = *ctx.output;
    // println!("{:?}", ctx.input.read_to_end()?);
    println!("{:?}", o2);
    */

    let mut ctx = StdIOContext {
        input: std::io::stdin(),
        output: std::io::stdout(),
    };
    ctx.write(&b"test"[..]);

    let mut mockctx = MockIOContext::new();
    mockctx.input.buf.push(123);
    let mut o: Vec<u8> = Vec::new();
    mockctx.read_to_end(&mut o)?;
    println!("{:?}", o);

    Ok(())
}
