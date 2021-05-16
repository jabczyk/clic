# Clic
A simple command-line interface calculator.
Written in [Rust](https://www.rust-lang.org/), powered by [meval](https://crates.io/crates/meval).

## Features
- Many built-in [functions and constants](https://github.com/rekka/meval-rs#supported-expressions)
- Shell mode
- Custom constants
- Customizable color scheme

## Usage
```bash
# evaluate an expression
clic "2+2"
= 4

# enter shell mode
clic
> set g 9.81
> 50 * g
= 490.5

# view help
clic help

```

## Development
```bash
cargo format && cargo clippy # lint
cargo build --release # build
```