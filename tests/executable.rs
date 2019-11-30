use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

struct TestCase<'a> {
    executable: Box<PathBuf>,
    args: Vec<&'a str>,
    stdin: Option<&'a str>,
    expected_stdout: Option<&'a str>,
    expected_stderr: Option<&'a str>,
    expected_retcode: i32,
}

impl<'a> TestCase<'a> {
    fn new() -> Self {
        let mut root: PathBuf = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        if root.ends_with("deps") {
            root.pop();
        };
        TestCase {
            executable: Box::new(root.join("bfi")),
            args: Vec::new(),
            stdin: None,
            expected_stdout: None,
            expected_stderr: None,
            expected_retcode: 0,
        }
    }

    fn with_arg(&mut self, arg: &'a str) -> &mut Self {
        self.args.push(arg);
        self
    }

    fn with_input(&mut self, input: &'a str) -> &mut Self {
        self.stdin = Some(input);
        self
    }

    fn expect_stdout(&mut self, stdout: &'a str) -> &mut Self {
        self.expected_stdout = Some(stdout);
        self
    }

    fn expect_stderr(&mut self, stderr: &'a str) -> &mut Self {
        self.expected_stderr = Some(stderr);
        self
    }

    fn expect_retcode(&mut self, retcode: i32) -> &mut Self {
        self.expected_retcode = retcode;
        self
    }

    fn execute(&self) {
        let mut child_proc = Command::new(&*self.executable)  // reref the deref
            .args(&self.args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("failed to execute");

        if let Some(s) = self.stdin {
            child_proc.stdin.as_mut()
                .expect("failed to open stdin")
                .write_all(s.as_bytes())
                .expect("failed to write to stdin");
        };
        let child_output = child_proc.wait_with_output().expect("failed to read stdout");

        let retcode = child_output.status.code().unwrap();
        assert_eq!(self.expected_retcode, retcode);

        if let Some(s) = self.expected_stdout {
            let stdout_str = std::str::from_utf8(&child_output.stdout).unwrap();
            assert_eq!(s, stdout_str);
        };

        if let Some(s) = self.expected_stderr {
            let stderr_str = std::str::from_utf8(&child_output.stderr).unwrap();
            assert_eq!(s, stderr_str);
        };
    }
}

#[test]
fn test_hello_world() {
    TestCase::new()
        .with_arg("+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.")
        .expect_stdout("Hello, World!")
        .execute();
}

#[test]
fn test_hello_world2() {
    TestCase::new()
        .with_arg("--unbuffered")
        .with_arg("
>++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<
+++>]<<]>-----.>->+++..+++.>-.<<+[>[+>+]>>]<--------
------.>>.+++.------.--------.>+.>+.")
        .expect_stdout("Hello World!\n")
        .expect_stderr("")
        .execute();
}

#[test]
fn test_squares() {
    let expected_result_vec: Vec<String> = (0..101).map(|i| (i * i).to_string()).collect();
    let mut expected_result = expected_result_vec.join("\n");
    expected_result.push('\n');
    TestCase::new()
        .with_arg("
++++[>+++++<-]>[<+++++>-]+<+[
    >[>+>+<<-]++>>[<<+>>-]>>>[-]++>[-]+
    >>>+[[-]++++++>>>]<<<[[<++++++++<++>>-]+<.<[>----<-]<]
    <<[>>>>>[>>>[-]+++++++++<[>-<-]+++++++++>[-[<->-]+[<<<]]<[>+<-]>]<<-]<<-
]
[Outputs square numbers from 0 to 10000.
Daniel B Cristofani (cristofdathevanetdotcom)
http://www.hevanet.com/cristofd/brainfuck/]")
        .expect_stdout(expected_result.as_str())
        .execute();
}

#[test]
fn test_cat() {
    let some_str: &str = "Some testing string!\n";
    TestCase::new()
        .with_arg(",[.[-],]")
        .with_input(some_str)
        .expect_stdout(some_str)
        .expect_stderr("")
        .execute();
}

// TODO: reimplement unicode support
#[test]
#[ignore]
fn test_cat_unicode() {
    let unicode_str: &str = "😸\n";
    TestCase::new()
        .with_arg("--utf8")
        .with_arg(",[.[-],]")
        .with_input(unicode_str)
        .expect_stdout(unicode_str)
        .execute();
}

// TODO: reimplement unicode support
#[test]
#[ignore]
fn test_cat_unicode_mangled() {
    let unicode_str: &str = "😸\n";
    TestCase::new()
        .with_arg(",[.[-],]")
        .with_input(unicode_str)
        .expect_stdout("\u{f0}\u{9f}\u{98}\u{b8}\n")  // mangled cat when each byte is ASCII decoded
        .expect_retcode(0)
        .execute();
}

// this is a little embarassing next to the BF program that prints the same output
fn sierpinski(n: u64) -> Vec<String> {
    if n == 0 {
        vec!["*".to_string()]
    } else {
        let prev = sierpinski(n - 1);
        let prev_width = prev.iter().rev().nth(0).unwrap().len();
        let next_width = prev_width * 2 + 1;
        let mut next: Vec<String> = Vec::new();
        for (i, cur) in prev.iter().enumerate() {
            let top = format!("{:>w$}", cur, w = cur.len() + ((next_width - prev_width) / 2));
            let bottom = format!("{}{:>w$}", cur, cur, w = next_width - prev_width);
            next.insert(i, top);
            next.push(bottom);
        }
        next
    }
}

#[test]
fn test_sierpinksi() {
    let sierpinski_string = format!("{}\n", sierpinski(5).join("\n"));
    println!("{}", sierpinski_string);
    TestCase::new()
        .with_arg("++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[-<<<[->[+[-]+>++
            >>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<]>.>+[>>]>+]")
        .expect_stdout(&sierpinski_string[..]) // ref to full length slice is the same as .as_str()
        .execute();
}

#[test]
fn test_missing_file() {
    TestCase::new()
        .with_arg("--file")
        .with_arg("does_not_exist.bf")
        .with_input("anything")
        .expect_retcode(1)
        .execute();
}

