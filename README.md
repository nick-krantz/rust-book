# The Rust Book

My general notes from reading [The Rust Book (Brown University)](https://rust-book.cs.brown.edu/)

## Chapter 1 - Intro

- methods that end in `!` are rust macros, different than functions: `println!`
- Compiling & running are separate steps
  - `rustc` complies source code, outputs a binary executable 
- `cargo` - build system &  package manager
  - `cargo new project_name` creates directory, toml file, and empty git repository 
  - TOML
    - [name] - brackets indicate a section heading
  - dependencies in rust are called crates
  - `cargo` expects that all source code lives within `src/`
  - `cargo build` builds/compiles a debug version (output: target/debug/*)
  - `cargo run` builds/compiles and runs the code
  - `cargo check` checks code to make sure it complies (faster than `cargo build`)
  - `cargo build --release` compiles code with optimizations (output: target/release/*)
  - `cargo doc --open` will build documentation for all dependencies and open it in a browser

## Chapter 2 - Guessing Game

- Variables are immutable by default
  - To make a variable mutable add `mut`
- `String::new`
  - `String` - type
  - `::new` - calls `new` method associated with that type
- `read_line(&mut guess)`
  - `&` marks that `guess` is a reference, `read_line` wouldn't need to copy the data of guess into new memory.
  - References are immutable by default hence: `&mut`
- `Result`
  - `Result` is an enum
    - Variants are `Ok` and `Err`
  - `Result::expect`
    - returned result is `Err`, causes the program to crash
    - returned result is `Ok`, just returns that value
    - Not using expect will cause a compile warning
- `println!("You guessed: {guess}");`
  - `{}` is a placeholder, holds value in place
  - `println!("x = {} and y = {}", x, y)` - replaces multiple placeholders
- Crates
  - Binary crate -> executable (program we're writing)
  - Library crate -> contains code intended to be used by other programs, cannot be executed on its own.
  - `rand = "0.8.3"` - "0.8.3" is shorthand for "^0.8.3"
- `start..=end`
  - builds a range expression inclusive
- `Ordering` enum
  - `Less`, `Greater`, `Equal`
- `cmp` 
  - method compares to values and can be called on anything that can be compared, taking a reference of what you want to compare with.
  - Returns an `Ordering` enum
- `match` arms
  - made up of "arms"
  - an "arm" consists of a "pattern" to match against, and the code that should run if the value fits the pattern.
  - Ends after the first successful match (no fricken fall-through)
- Rust defaults numbers to `i32` type
- Rust allows you to shadow variables
  - declaring variables of the same name is okay
- `loop` keyword creates an infinite loop
