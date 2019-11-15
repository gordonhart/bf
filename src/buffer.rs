use std::io::Write;

pub trait Buffer {
    fn put_byte(&self, byte: u8);
}

impl std::fmt::Debug for dyn Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "buffer")
    }
}

impl PartialEq for dyn Buffer {
    fn eq(&self, _other: &Self) -> bool {
        true  // TODO: what should this look like?
    }
}

#[derive(Debug, PartialEq)]
pub struct ASCIICharBuffer {}
impl Buffer for ASCIICharBuffer {
    fn put_byte(&self, byte: u8) {
        print!("{}", byte as char);
        std::io::stdout().flush().expect("unable to flush stdout");
    }
}

#[derive(Debug, PartialEq)]
pub struct ASCIILineBuffer {}
impl Buffer for ASCIILineBuffer {
    fn put_byte(&self, byte: u8) {
        let c = byte as char;
        print!("{}", c);
        if c == '\n' {
            std::io::stdout().flush().expect("unable to flush stdout");
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UTF8CharBuffer {
    buffer: Vec<u8>,
}
/*
impl Buffer for UTF8CharBuffer {
    fn put_byte(&self, byte: u8) {
        self.buffer.push(byte);
        match std::str::from_utf8(&self.buffer) {
            Ok(c) => {
                print!("{}", c);
                std::io::stdout().flush().expect("unable to flush stdout");
                self.buffer.truncate(0);
            },
            Err(_) => {},
        }
    }
}
*/

#[derive(Debug, PartialEq)]
pub struct FileBuffer {}
