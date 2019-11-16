use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

struct TestContext<'a> {
    executable: Box<PathBuf>,
    args: Vec<&'a str>,
    stdin: Option<&'a str>,
    expected_stdout: Option<&'a str>,
    expected_stderr: Option<&'a str>,
    expected_retcode: i32,
}

impl<'a> TestContext<'a> {
    fn new() -> Self {
        let mut root: PathBuf = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        if root.ends_with("deps") {
            root.pop();
        };
        TestContext {
            executable: Box::new(root.join("bf")),
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
    TestContext::new()
        .with_arg("+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.")
        .expect_stdout("Hello, World!")
        .execute();
}

#[test]
fn test_hello_world2() {
    TestContext::new()
        .with_arg("
>++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<
+++>]<<]>-----.>->+++..+++.>-.<<+[>[+>+]>>]<--------
------.>>.+++.------.--------.>+.>+.")
        .expect_stdout("Hello World!\n")
        .execute();
}

#[test]
fn test_squares() {
    let expected_result_vec: Vec<String> = (0..101).map(|i| (i * i).to_string()).collect();
    let mut expected_result = expected_result_vec.join("\n");
    expected_result.push('\n');
    TestContext::new()
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
    TestContext::new()
        .with_arg(",[.[-],]")
        .with_input(some_str)
        .expect_stdout(some_str)
        .expect_stderr("")
        .execute();
}

#[test]
fn test_cat2() {
    let unicode_str: &str = "😸\n";
    TestContext::new()
        .with_arg("--utf8")
        .with_arg(",[.[-],]")
        .with_input(unicode_str)
        .expect_stdout(unicode_str)
        .execute();
}

#[test]
fn test_unicode_failure() {
    let unicode_str: &str = "😸\n";
    TestContext::new()
        .with_arg(",[.[-],]")
        .with_input(unicode_str)
        .expect_stdout("ð\u{9f}\u{98}¸\n")  // mangled cat when each byte is ASCII decoded
        .expect_retcode(0)
        .execute();
}
