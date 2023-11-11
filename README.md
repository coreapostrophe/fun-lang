# 🔣 funlang
> ⚠️ This project is still a work in progress

## About
"Fun" is a function-centric language written and interpreted in Rust. Its main use is for creating collections of minute closure definitions. This allows users to configure small functions that could then be used in configurations without ever writing a program.

## Crates
- [`funlang`](/crates/funlang/) - core implementation of the language's interpreter; contains its lexer and parser module.
- [`funlang_derive`](/crates/funlang_derive/) - houses all the procedural macros used for the core crate.
- [`funlang_error`](/crates/funlang_error/) - contains all the shared structures for the language's error handling.
