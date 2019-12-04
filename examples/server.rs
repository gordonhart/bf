extern crate bfi;
extern crate zmq;

use std::io::Write;
use std::cell::RefCell;
use std::io;

use bfi::{ioctx, interpreter};

struct ServerIoCtx {
    ctx: zmq::Context,
    socket: zmq::Socket,
    input_buffer: Vec<u8>,
    readable: bool,
}

impl ioctx::IoCtx for ServerIoCtx {
    fn read_input(&mut self, mut buf: &mut [u8]) -> Result<usize, io::Error> {
        let flags = 0i32;
        if self.input_buffer.len() == 0 {
            if !self.readable {
                // TODO: better error
                return Err(io::Error::new(io::ErrorKind::Other, "not readable"))
            } else {
                match self.socket.recv_bytes(flags) {
                    Ok(bytes_received) => {
                        buf[0] = *bytes_received.iter().next().unwrap();
                        self.input_buffer.extend_from_slice(&bytes_received[1..]);
                        println!("received {} bytes", bytes_received.len());
                        self.readable = false;
                        Ok(1)
                    }
                    Err(_) => {
                        println!("unable to read input");
                        Err(io::Error::new(io::ErrorKind::Other, "unable to read"))
                    }
                }
            }
        } else {
            buf.write(&self.input_buffer[..1])?;
            self.input_buffer.remove(0);
            Ok(1)
        }
    }

    fn write_output(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let flags = 0i32;
        if self.readable {
            return Err(io::Error::new(io::ErrorKind::Other, "toggled to read, can't send"));
        };
        match self.socket.send(buf, flags) {
            Ok(_) => {
                println!("sent {} bytes", buf.len());
                self.readable = true;
                Ok(buf.len())
            },
            Err(_) => {
                println!("unable to write output");
                Err(io::Error::new(io::ErrorKind::Other, "unable to send"))
            }
        }
    }

    fn flush_output(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

impl ServerIoCtx {
    fn new(protocol: &str, address: &str, port: u32) -> Self {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::REP).unwrap();
        let address: String = format!("{}://{}:{}", protocol, address, port);
        socket.bind(address.as_str()).unwrap();
        Self {
            ctx: ctx,
            socket: socket,
            input_buffer: Vec::new(),
            readable: true,
        }
    }
}

fn main() {
    let adder: &str = "+[-,>,<[->+<]>.[-]<+]";
    let ictx = ServerIoCtx::new("tcp", "127.0.0.1", 12345u32);
    let ictx_rc = RefCell::new(Box::new(ictx) as Box<dyn ioctx::IoCtx>);
    let status = interpreter::ExecutionContext::new(ictx_rc.borrow_mut(), adder).execute();
    println!("exit status: {:?}", status);
}
