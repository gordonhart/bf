# Brainfuck Interpreter
The secondary purpose of this project is to implement an interpreter for the
[Brainfuck programming language](https://en.wikipedia.org/wiki/Brainfuck) with
a few extensions. The primary purpose is to learn Rust.



## Usage
The interpreter can be compiled and run:
```
$ cargo run <program>
```
Or compiled first then run:
```
$ cargo build --release
$ ./target/release/bf <program>
```



## Language Extensions
This implementation follows the
[Wikipedia standard](https://en.wikipedia.org/wiki/Brainfuck#Commands) with
some additions:
- `?`: Dump the internal program execution state to stderr
- `!`: Breakpoint to enter into a Brainfuck REPL



## Goals
- No `panic!`
- Idiomatic language usage
- Correct BF implementation
- Standard command line niceties:
    - Option to run program from argument
    - Option to run program from file
    - Read program input from stdin
    - Proper exit code setting
    - Expected options like `--help`
- Reasonable performance and resource usage (nothing dumb)



## Example Programs
1. `hello world`:
```
+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.
```

2. `Hello World!`:
```
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+
++.------.--------.>>+.>++.
```

3. `Hello World!`:
```
>++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->+++..+++.
>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+.
```

4. `Hello, World!`:
```
+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.
```

5. `cat` implementation:
```
,[.[-],]
```

    - Example: `$ ./bf ',[.[-],]' < README.md`

## TODOs
- [ ] Accept and ignore non-command characters instead of failing (comments)
- [ ] Integrate `readline` for REPL input
- [ ] Write docstrings
- [ ] Support running from file with `-f` and `--file`
- [ ] Add `-h`/`--help` usage flag
- [ ] Add `-v`/`--verbose` flag to print extra information (like exit message)
