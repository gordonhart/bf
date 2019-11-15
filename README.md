# BrainF\*ck Interpreter
[![Build Status](https://travis-ci.com/gordonhart/bf.svg?branch=master)](https://travis-ci.com/gordonhart/bf)

The secondary purpose of this project is to implement an interpreter for the
[BrainF\*ck programming language](https://en.wikipedia.org/wiki/Brainfuck) with
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
- `!`: Breakpoint to enter into a BrainF\*ck REPL



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
- Non-negligible test coverage
- No compiler warnings



## Example Programs
1. `Hello, World!`:
```
+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.
```

2. `cat` implementation:
```
,[.[-],]
```

    - Example: `$ ./bf ',[.[-],]' < README.md`

3. Display the 5th iteration of the
[Sierpinski Triangle](http://www.hevanet.com/cristofd/brainfuck/):
```
++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[
    -<<<[
        ->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<
    ]>.>+[>>]>+
]
```


## TODOs
- [x] Accept and ignore non-command characters instead of failing (comments)
- [x] Integrate `readline` for REPL input
- [ ] Write docstrings
- [x] Support running from file with `-f` and `--filename`
- [x] Add `-h`/`--help` usage flag
- [x] Add `-v`/`--verbose` flag to print extra information (like exit message)
- [ ] Implement full unit test coverage
- [x] Apply `rustfmt` formatting
- [ ] Support unicode output with `-u`/`--utf8` flag
