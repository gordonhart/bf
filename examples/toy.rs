extern crate bfi;

// Run with `cargo run --example toy`
fn main() {
    let program: &str = ",[.[-],]";
    let input: &[u8] = b"toy test!";
    match bfi::execute(program, input) {
        Ok(output_vec) => {
            let output_str: &str = std::str::from_utf8(output_vec.as_slice()).unwrap();
            println!("program output: {}", output_str);
        },
        Err(e) => eprintln!("error: {:?}", e),
    };
}
