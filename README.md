# Interactive BrainF\*ck Interpreter
[![Build Status](https://travis-ci.com/gordonhart/bfi.svg?branch=master)](https://travis-ci.com/gordonhart/bfi)
[![codecov](https://codecov.io/gh/gordonhart/bfi/branch/master/graph/badge.svg)](https://codecov.io/gh/gordonhart/bfi)

Are you tired of writing Python and JS at your day job? Looking to learn a
lower-level language but dreading leaving the comforts of your REPL? Well, your
search is over: [BrainF\*ck](https://en.wikipedia.org/wiki/Brainfuck) is your
language and `bfi` is your interpreter.

Once you bite the bullet, jump in, and get acclimated, you'll wonder why you
ever waited. Without built-in support for
variable names, a garbage collector, concurrency, syscalls, literals, objects,
inheritance, types, syntax, floating point, keywords, a stack, a heap, best
practices, or Stack Overflow there are almost no ways left for you to shoot
yourself in the foot.

Of course, if you're feeling a moment of human weakness, it would be possible
to implement any of those things. I'm not sure who this Turing guy/gal was but
I heard s/he called this language complete.


## The Basics

The ethos of BrainF\*ck is to leave difficult things like critical thinking and
analysis to the machines like [GPT-2](https://github.com/openai/gpt-2) that are
good at them and focus only on stuff you, as a human, are naturally good at:
pointer arithmetic, rigid control flow, and data encoding.

Imagine you're back on the primordial savannah but instead of a spear in your
hand it's a pointer and instead of lush grassland around you it's a single
dimensional roll of tape extending forever off into the horizon. If you have
properly engaged your lizard brain this scene should come naturally.

You have 8 standard language commands to memorize:

| Command | Description |
| ------- | ----------- |
| `+` | Increment the current cell (with rollover) |
| `-` | Decrement the current cell (will rollunder) |
| `>` | Move to the next cell |
| `<` | Move to the previous cell |
| `.` | Output the current cell |
| `,` | Read input into the current cell |
| `[` | If the current cell is zero, skip ahead to the matching `]` |
| `]` | If the current cell is not zero, skip back to the matching `[` |

Plus two more `bfi` extension commands:

| Command | Description |
| ------- | ----------- |
| `?` | Dump program internals to `stderr` |
| `!` | Enter into a REPL |


## Example Programs
Now that you have the language down pat we can jog our legs with a couple of
gumball programs:

1. `Hello, World!`:
```
+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.
```

2. `cat` implementation:
```
,[.[-],]
```
- Example: `$ <README.md bfi ',[.[-],]'`

3. Display the 5th iteration of the
[Sierpinski Triangle](http://www.hevanet.com/cristofd/brainfuck/):
```
++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[
    -<<<[
        ->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<
    ]>.>+[>>]>+
]
```


---

Serious face this time: the secondary purpose of this project is the
interpreter; the primary purpose is to learn Rust.


## Usage
The interpreter can be compiled and run:
```
$ cargo run <program>
```
Or compiled first then run:
```
$ cargo build --release
$ ./target/release/bfi <program>
```


## Objectives
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


## TODOs
- [x] Accept and ignore non-command characters instead of failing (comments)
- [x] Integrate `readline` for REPL input
- [ ] Write docstrings
- [x] Support running from file with `-f` and `--filename`
- [x] Add `-h`/`--help` usage flag
- [x] Add `-v`/`--verbose` flag to print extra information (like exit message)
- [ ] Implement full test coverage
- [x] Apply `rustfmt` formatting
- [x] Support unicode output with `-u`/`--utf8` flag
- [ ] Implement direct-to-file output with `-o`/`--output` flag
