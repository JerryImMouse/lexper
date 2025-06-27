# lexper
[![Crates.io](https://img.shields.io/crates/v/rexpr.svg)](https://crates.io/crates/lexper)
[![Docs.rs](https://docs.rs/rexpr/badge.svg)](https://docs.rs/lexper)  
lexper is a small & lightweight calculator written in Rust with custom lexer and parser.  
It also supports constants and function calls(currently sin() and PI as constant).

## Installation
Just use `cargo add lexper` or add the following line to your Cargo.toml:
```toml
[dependencies]
lexper = "*" # use the latest version or "*" for the latest
```

## Usage
lexper can be used by 3 ways.
- Manually creating Lexer, Parser and Context objects and then passing data through them
- Using lexper::eval(), same as above, just less boilerplate
- Using lexper::eval!, same as above, but with an ability to format input with `{} + 2` 

Here is the most simple one, the last.
```rust
use lexper;

fn main() {
    let result = lexper::eval!("{} + 2", 5).unwrap();
    println!("{}", result);
}
```

## Known Issues
- Some expressions like "(2 + 2" will work, though the left parenthesis isn't closed, i'll fix that in time.
- There are no factorial support, it'll be added in future updates
- Lack of builtin functions and ability to add them yourself. It will be implemented in future updates
