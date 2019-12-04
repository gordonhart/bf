# Interactive BrainF\*ck Interpreter
[![Build Status](https://travis-ci.com/gordonhart/bfi.svg?branch=master)](https://travis-ci.com/gordonhart/bfi)
[![codecov](https://codecov.io/gh/gordonhart/bfi/branch/master/graph/badge.svg)](https://codecov.io/gh/gordonhart/bfi)

Are you tired of writing Python and JS at your day job? Looking to learn a
lower-level language but dreading leaving the comforts of your REPL? Well, your
search is over: [BrainF\*ck](https://en.wikipedia.org/wiki/Brainfuck) is your
language and `bfi` is your interpreter.

Once you bite the bullet, jump in, and get acclimated, you'll wonder why you
ever waited. Without built-in support for variable names, a garbage collector,
concurrency, syscalls, literals, objects, inheritance, types, syntax, floating
point, keywords, data structures, best practices, or Stack Overflow there are
almost no ways left for you to shoot yourself in the foot.

Of course, if you're feeling a moment of human weakness, it would be possible
to implement any of those things.


## The Basics

The ethos of BrainF\*ck is to leave difficult things like critical thinking and
analysis to the machines like [GPT-2](https://github.com/openai/gpt-2) that are
good at them and focus only on stuff you, as a human, are naturally good at:
pointer arithmetic, rigid control flow, and data encoding.

Imagine you're back on the primordial savannah but instead of a spear in your
hand it's a pointer and instead of lush grassland around you it's a single
dimensional roll of tape extending forever off into the point horizon. If you
have properly engaged your lizard brain this scene should come naturally.

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
| `#` | Dump program internals to `stderr` |
| `%` | Enter into a REPL |


## Example

Now that you have the language down pat we can jog our legs with a gumball
program:

```
,[.[-],]
```

Recognize it? It's our friend, `cat`. It reads from stdin and spits it out to
stdout until a NUL terminator is received. My productivity skyrocketed after
aliasing 😸 to it in my shell. Not having to worry about a `cat` implementation
I don't understand freed up a ton of headspace better spent visualizing tape.
Who _really_ needs options like `--help` from their command line utilities?

Example usage:

```
$ <README.md bfi ',[.[-],]'
```

Yes, GNU `cat` supports interior NUL bytes and this program does not. Go away.


# `bfi` as a Library

Luckily for you Rust programmers, `bfi` has a library interface! See
`examples/toy.rs` for a starting point.

## Foreign Usage

Some hope remains for those of us forced to use a tired language like Python
in our professionial environments. Check your despair at the gate, we're about
to embark on a fantastical journey to the foreign land of `bfi` + FFI.

Well, not so fantastical. You can work with `libbfi` the same way you'd
incorporate any foreign object into your project. See
`examples/python/bindings.py` for a Python integration using `ctypes`.

Further, `examples/python/trick_your_boss.py` contains a minimal framework for
surreptitiously programming in BrainF\*ck at work under your manager's nose.
Don't worry about the resulting proliferation of binary blobs in your repo, odds
are s/he doesn't even review your code. On the off chance you get a question
about all of the "corrupt GIFs" appearing, make something up about the pixel
depth or proprietary codecs or just let GPT-2 make your excuse up for you.


---
---
---


Serious face this time: the secondary purpose of this project is the
interpreter; its primary purpose is as a playground to learn Rust.


## Objectives

- Idiomatic Rust language usage
- Correct BF implementation
- Standard command line niceties:
    - Option to run program from argument
    - Option to run program from file
    - Read program input from stdin
    - Proper exit code setting
    - Expected options like `--help`
    - Expected behavior on SIGINT, EOF, SIGTERM, etc.
- Reasonable performance and resource usage (nothing dumb)
- Reasonable usage of `panic!` (and things that cause it, like `unwrap`)
- Non-negligible test coverage
- No compiler warnings
- No [clippy](https://github.com/rust-lang/rust-clippy) warnings


## TODOs

- [x] Accept and ignore non-command characters instead of failing (comments)
- [x] Integrate `readline` for REPL input
- [x] Write docstrings
- [x] Support running from file with `-f` and `--filename`
- [x] Add `-h`/`--help` usage flag
- [x] Add `-v`/`--verbose` flag to print extra information (like exit message)
- [x] Implement close-to-full test coverage
- [x] Apply `rustfmt` formatting
- [x] Document all `panic!` cases with a `# Panics` docstring section
