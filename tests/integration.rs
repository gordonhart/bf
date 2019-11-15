use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

// static EXE: &'static PathBuf = get_exe();

fn get_exe() -> PathBuf {
    let mut root = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    if root.ends_with("deps") {
        root.pop();
    };
    root.join("bf")
}

fn test_program(prog: &str, input: &str, output: &str) {
    let mut child = Command::new(&get_exe())
        .arg(prog)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to execute");

    let stdin = child.stdin.as_mut().expect("failed to open stdin");
    stdin
        .write_all(input.as_bytes())
        .expect("failed to write to stdin");
    let child_output = child.wait_with_output().expect("failed to read stdout");

    let retcode = child_output.status.code().unwrap();
    assert_eq!(0, retcode);
    let stdout_str = std::str::from_utf8(&child_output.stdout).unwrap();
    assert_eq!(output, stdout_str);
}

#[test]
fn test_hello_world() {
    test_program(
        "+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.",
        "",
        "Hello, World!",
    );
}

#[test]
fn test_hello_world2() {
    test_program(
        "
>++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<
+++>]<<]>-----.>->+++..+++.>-.<<+[>[+>+]>>]<--------
------.>>.+++.------.--------.>+.>+.",
        "",
        "Hello World!\n",
    );
}

#[test]
fn test_squares() {
    let expected_result_vec: Vec<String> = (0..101).map(|i| (i * i).to_string()).collect();
    let mut expected_result = expected_result_vec.join("\n");
    expected_result.push('\n');
    test_program(
        "
++++[>+++++<-]>[<+++++>-]+<+[
    >[>+>+<<-]++>>[<<+>>-]>>>[-]++>[-]+
    >>>+[[-]++++++>>>]<<<[[<++++++++<++>>-]+<.<[>----<-]<]
    <<[>>>>>[>>>[-]+++++++++<[>-<-]+++++++++>[-[<->-]+[<<<]]<[>+<-]>]<<-]<<-
]
[Outputs square numbers from 0 to 10000.
Daniel B Cristofani (cristofdathevanetdotcom)
http://www.hevanet.com/cristofd/brainfuck/]",
        "",
        expected_result.as_str(),
    );
}

#[test]
fn test_cat() {
    let some_str: &str = "Some testing string!\n";
    test_program(",[.[-],]", some_str, some_str);
}

#[test]
#[ignore]
fn test_cat2() {
    test_program(",[.[-],]", "😸", "😸");
}
