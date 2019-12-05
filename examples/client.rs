extern crate clap;
extern crate zmq;

use clap::{App, Arg};

// client to the example server, usage:
// `$ cargo run --example client 10 12`
fn main() -> Result<(), zmq::Error> {
    let args = App::new("a + b")
        .arg(Arg::with_name("a").index(1).required(true))
        .arg(Arg::with_name("b").index(2).required(true))
        .get_matches();

    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REQ)?;
    socket.connect("tcp://127.0.0.1:12345")?;

    let a: u8 = args.value_of("a").unwrap().parse().expect("arg \"a\" error");
    let b: u8 = args.value_of("b").unwrap().parse().expect("arg \"b\" error");
    let input = vec![a, b];
    socket.send(input, 0i32)?;

    let mut result_buffer = [0u8; 1];
    socket.recv_into(&mut result_buffer, 0i32)?;
    println!("{} + {} = {}", a, b, result_buffer[0]);

    Ok(())
}
