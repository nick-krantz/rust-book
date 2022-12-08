# The Rust Book

My general notes from reading [The Rust Book (Brown University)](https://rust-book.cs.brown.edu/)

## Chapter 1 - Intro

- methods that end in `!` are rust macros, different than functions: `println!`
- Compiling & running are separate steps
  - `rustc` complies source code, outputs a binary executable
- `cargo` - build system & package manager
  - `cargo new project_name` creates directory, toml file, and empty git repository
  - TOML
    - [name] - brackets indicate a section heading
  - dependencies in rust are called crates
  - `cargo` expects that all source code lives within `src/`
  - `cargo build` builds/compiles a debug version (output: target/debug/\*)
  - `cargo run` builds/compiles and runs the code
  - `cargo check` checks code to make sure it complies (faster than `cargo build`)
  - `cargo build --release` compiles code with optimizations (output: target/release/\*)
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

## Chapter 3 - Common Programming Concepts

### Constants
- not allowed to use `mut`
- can be declared in the global scope
- the type must be annotated
- can only be set to a constant expression, no the result of a value that could be computed at runtime.
- constants are valid the entire length of the program within the scope they were declared in.

### Shadowing
- declare a new variable with the same name as a previous variable
- Shadowing allows you to have an immutable variable but still update the value
- You can change the type of the variable but reuse the same name

### Data Types
- Two data subsets: scalar & compound

#### Scalar type
- represents a single value
- 4 primary scalar types: integers, floating-point numbers, Booleans, characters.
- Integer - number without a fractional component
  - Integer types:
    | Length | Signed | Unsigned |
    | ------ | ------ | -------- |
    | 8-bit | i8 | u8 |
    | 16-bit | i16 | u16 |
    | 32-bit | i32 | u32 |
    | 64-bit | i64 | u64 |
    | 128-bit | i128 | u128 |
    | arch | isize | usize |
  - Signed numbers stored with two's complement: -(2<sup>n - 1</sup>) to 2<sup>n - 1</sup> - 1
  - Unsigned numbers can store: 0 to 2<sup>n - 1</sup>
  - `isize` & `usize` - depend on the architecture of the computer that the program is run on. 64 or 32 bit.
  - Numbers allow for suffix to show type: `57u8`
  - Number literal can use `_` as a visual separator: `1_000_000` is the same as `1000000`
  - Integer Overflow
    - In debug mode, rust will check for possible integer overflows
    - In release mode, rust does not check for overflow, Rust performs two's compliment wrapping. Overflow "wraps" around to the lowest number: in a `u8` 256 becomes 0, 257 becomes 1. This is bad.
    - To handle use one of the 4 methods from the standard library: `wrapping_*`, `checked_*`, `overflowing_*`, `saturating_*`
- Floating-Point Types
  - `f32` - single precision
  - `f64` - double precision
- Integer division rounds down to the nearest integer
- Boolean
  - `true` & `false`
  - One byte in size
  - specified by `bool`
- Character Type
  - uses single quotes
  - string literals use double quotes
  - 4 bytes
  - represents a Unicode Scalar Value (way more than just ASCII)

#### Compound Types
- Multiple values into one type
- Tuple & Arrays
- Tuple
  - Comma separated list inside of parentheses
  - deconstruct the tuple using `let`
  - values can be accessed directly by using `.` then the index: `tup.2`
  - Tuple without any values is called a unit: `()`
- Array
  - Every element of the array must have the same type
  - Has a fixed length
  - Comma separated values in square brackets
  - Useful when data should be allocated on the stack rather than the heap
  - arrays can be typed by the value type followed by the number of values: `[i32; 5]` is an array of 5 `i32` values.
  - arrays can be initialized with the same value by: `[4, 5]` will be an array of length 5, all filled with the value of 4.
  - array values can be access with square brackets
  - index out of bounds will cause a panic
  
### Functions

- snake case for function (and variable names)
- `fn` keyword for function declarations
- order of declared functions doesn't matter, but they
- types have to be defined on function parameters

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resulting value.
  - expressions to do not include ending semicolons
  
- Return values
  - declared with an arrow and a type: `-> i32`
  ```rust
  fn five() -> i32 {
      5
  }
  ```
### Comments

// Double slash

### Control Flow

- `if` has no parentheses
  - condition must be a `bool`, no truthy or falsy
  - `else` & `else if` work as you'd expect
- `if` can be used like a javascript ternary:
  `let number = if condition { 5 } else { 6 };`
  - results from each arm of the `if` must be of the same type in the above result
