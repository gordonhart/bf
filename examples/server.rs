extern crate bfi;
extern crate zmq;

use std::io::Write;
use std::cell::RefCell;
use std::io::{self, Error, ErrorKind};

use bfi::{ioctx, interpreter};

#[derive(Debug)]
enum SocketStatus {
    Readable,
    Writable,
}

struct ZmqRepServerIoCtx {
    socket: zmq::Socket,
    input_buffer: Vec<u8>,
    status: SocketStatus, // REQ/REP pair is strictly alternating
}

impl ZmqRepServerIoCtx {
    fn new(protocol: &str, address: &str, port: u32) -> Self {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::REP).unwrap();
        let address: String = format!("{}://{}:{}", protocol, address, port);
        socket.bind(address.as_str()).unwrap();
        Self {
            socket: socket,
            input_buffer: Vec::new(),
            status: SocketStatus::Readable,
        }
    }
}

impl ioctx::IoCtx for ZmqRepServerIoCtx {
    fn read_input(&mut self, mut buf: &mut [u8]) -> Result<usize, Error> {
        let flags = 0i32;
        match (self.input_buffer.len(), &self.status) {
            // ran out of input bytes, which is a caller problem
            (0, SocketStatus::Writable) => {
                // can't do anything, caller needs to send two bytes next time
                Ok(0)
            },
            // dig into our store of previously received bytes that have yet to be consumed
            (_, SocketStatus::Writable) => {
                buf.write(&self.input_buffer[..1])?;
                self.input_buffer.remove(0);
                Ok(1)
            },
            (_, SocketStatus::Readable) => {
                match self.socket.recv_bytes(flags) {
                    // empty packet received, report zero bytes read and have faith that this is
                    // handled properly by the bf program (it is)
                    Ok(v) if v.len() == 0 => {
                        self.status = SocketStatus::Writable;
                        Ok(0)
                    },
                    // write the first byte to the buffer, store the rest for later access
                    Ok(bytes_received) => {
                        buf.write(&bytes_received[..1])?;
                        self.input_buffer.extend_from_slice(&bytes_received[1..]);
                        self.status = SocketStatus::Writable;
                        Ok(1)
                    }
                    // bad news if we reach here
                    Err(e) => Err(Error::new(ErrorKind::Other, format!("{:?}", e))),
                }
            },
        }
    }

    fn write_output(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let flags = 0i32;
        match &self.status {
            SocketStatus::Writable => {
                match self.socket.send(buf, flags) {
                    Ok(()) => {
                        self.status = SocketStatus::Readable;
                        // any input left is garbage extra input from the caller
                        self.input_buffer.clear();
                        Ok(buf.len())
                    },
                    // fatal
                    Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))),
                }
            },
            // shouldn't occur due to the simplicity of the bf program and protections in read_input
            SocketStatus::Readable => Err(Error::new(ErrorKind::Other, "unable to write")),
        }
    }

    fn flush_output(&mut self) -> Result<(), io::Error> {
        // has no meaning with ZMQ sockets
        Ok(())
    }
}

fn main() {
    // reads two input bytes and returns their wrapped sum, nonterminating
    let adder: &str = "+[-,>,<[->+<]>.[-]<+]";
    let ictx = ZmqRepServerIoCtx::new("tcp", "127.0.0.1", 12345u32);
    let ictx_rc = RefCell::new(Box::new(ictx) as Box<dyn ioctx::IoCtx>);
    let status = interpreter::ExecutionContext::new(ictx_rc.borrow_mut(), adder).execute();
    println!("exit status: {:?}", status);
}
