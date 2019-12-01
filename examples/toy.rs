extern crate bfi;


fn main() {
    let program: &str = ",[.[-],]";
    let input: &[u8] = b"toy test!";
    match bfi::execute(program, input) {
        Ok(output_vec) => println!("output: {:?}", output_vec),
        Err(e) => eprintln!("error: {:?}", e),
    };
}
