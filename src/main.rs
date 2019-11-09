use std::env;

mod bf;

fn main() {
    /*
    println!("{}", bf::Action::PtrInc);
    println!("{}", bf::decode_action(bf::Action::PtrDec));
    println!("{}", bf::decode_action(bf::Action::ValInc));
    */

    let args: Vec<String> = env::args().collect();
    // assert_eq!(args.len(), 2);
    match &args[..] {
        [_, arg] => println!("{}", arg),
        [_] => eprintln!("missing arg"),
        _ => eprintln!("too many"),
    }
}
