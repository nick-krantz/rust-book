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


#### Conditional

- `if` has no parentheses
  - condition must be a `bool`, no truthy or falsy
  - `else` & `else if` work as you'd expect
- `if` can be used like a javascript ternary:
  `let number = if condition { 5 } else { 6 };`
  - results from each arm of the `if` must be of the same type in the above result
  
#### Loops

- 3 types of loops: `loop`, `while`, `for`
- `loop` - executes loop forever until explicitly stopped.
  - `break` will stop the loop
  - `continue` will skip over any remaining code in the iteration and start the loop again.
  - `loop`s are an expression they can return a value
- `while` & `for` work as you'd expect

## Chapter 4 - Ownership

- How a Rust program manages memory
  - Some languages have a garbage collector that runs and frees memory
  - Some languages the developer has to allocate and then free memory manually
- If the checks of ownership fail, the program won't compile

### Stack & Heap

- The stack & the heap are both available memory at runtime
- The stack stores values in the order that it gets them and removes the values in opposite order: LIFO.
- The heap is less organized as it you request a certain amount of memory and a pointer is returned of the address
- The stack is faster because there doesn't need to be a search for a place to store the data. The heap needs to find a space big enough for the new data.
  - Same with accessing data, the pointer must be followed to read the data.

### Ownership Rules

1. Each value in Rust has an owner
1. There can only be one owner at a time
1.  When the owner goes out of scope, the value will be dropped.

- `String` example
  - Complex data type unlike: integers, floating-point numbers, Booleans, characters
  - Stored on the heap
  - create a `String` from string literal: `let s = String::from("hello");`
  - `::` references a method under a namespace
  - String literals are known at compile time so their size is known, the text is hardcoded directly into the final executable.
  - `String` type are mutable so they allocate memory in the heap. `String::from` requests the memory it needs
  - When a variable goes out of scope rust calls `drop` which will un-allocate the memory.

- `Clone`
  - `.clone()` deep copies heap data as well as stack data
 
- `Copy`
  - `.copy()` - for values that are stored on the stack
  - nothing that requires allocation can implement `Copy`.
  - integers, floating-point numbers, Booleans, characters implement `copy`
  - tuples can implement `copy` if they only contain types that implement `copy`
 
- Functions
  - Passing a variable to a function will `move` or `copy` ownership just as an assignment does.
  - returning values can transfer ownership as well
  
### References and Borrowing

- A reference is like a pointer in that it's an address we can follow to access the data stored at that address, but that data is owned by another variable.
- Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
- References are denoted with `&`
- The opposite, a deference is denoted by `*`
- references are immutable by default
  - references can be made mutable by `&mut`
- If you have a mutable reference to a a value, you can have no other references to that value.
  - This prevents data races, which can happen when:
    - Two or more pointers access the same data at the same time
    - At least one of the pointers is used to write to the data
    - There's no mechanism being used to synchronize the data.
- You cannot have a mutable reference while there is a immutable reference to the same value in scope

### Slice

- A reference to a contiguous sequence of elements in a collection (rather than the whole collection).
- Does not have ownership
- string slice type is `&str`
  - string literals are slices
  - `let s = "Hello, world!";` is type `&str`
- You can use slices with other data types like arrays

## Chapter 5 - Structs

- Object of attributes, usually related values
  - similar to `type` or `interface` in TypeScript
- keyword: `struct`
- Create an instance by stating the name of the struct, then curly brackets with each key value pair of the struct within it.
- To get a specific value from a struct, dot notation: `user.name`
- An entire struct is either all immutable or all mutable. You cannot have only certain fields of a struct be immutable or mutable.
- Tuple Structs
  - Provides a name to a set a values but does not use key-value pairs
  - `struct Color(i32, i32, i32);`
- Unit-Like Structs
  - `struct AlwaysEqual` - useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. (more to come)
- Printing struct to screen
  - add attribute`#[derive(Debug)]` to struct
  - `println!` with `{:?}` for inline
  - `println!` with `{:#?}` for pretty-print
  - `dbg!(&struct_name)` returns ownership but also prints the line number
- Methods
  - defined within the context of a struct (or enum or trait)
  - first parameter is always `self`, the instance of the struct they're being called on.
  - start with `impl` then the struct name: `impl Rectangle {}`
  - Methods can take ownership of `self` so `&self` is common.
  - Methods can have the same name as fields on the struct
  - use dot notation to invoke
- Associated Functions
  - defined within the context of a struct (or enum or trait)
  - do not take `self` as a first parameter
  - use `::` to invoke
  - use `impl` just as in methods
  
