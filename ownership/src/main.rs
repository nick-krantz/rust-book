fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // ownership with variables & data interaction
    {
        let x = 5; // assign value 5 to x
        let y = x; // make a copy of value x and assign to y
                   // this is efficient because 5 is a known, fixed size

        let s1 = String::from("hello");
        // creates string "hello" on the heap,
        // 3 things are stored on the stack:
        // 1. the pointer to the first memory index
        // 2. the length (in bytes) that is being used
        // 3. the capacity (in bytes)
        let s2 = s1;
        // all of the data on the stack is coped from `s1` into `s2`
        // the underlying data on the heap isn't copied just the pointer, length and capacity.

        // println!("{}, world!", s1);
        // ^ borrow after move error, rust invalidate s1
        // after the `let s2 = s1` assignment, s1 was "moved",
        // to s2.

        println!("s2 = {s2}");

        let s3 = String::from("hello");
        let s4 = s3.clone(); // clones all heap data as well, expensive

        println!("s3 = {}, s4 = {}", s3, s4);
    }

    {
        let s = String::from("hello");

        takes_ownership(s); // `s` value moves into `takes_ownership`
                            // using `s` beyond here would be invalid

        let x = 5;

        makes_copy(x); // `x` moves into `makes_copy`
                       // because `x` is i32 `Copy` is used,
                       // `x` is still valid beyond here
    }

    {
        let s1 = gives_ownership(); // s1 comes into scope from `gives_ownership`

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 moves into method, while bringing s3 into scope
    } // s3 & s1 go out of scope
}

fn takes_ownership(some_string: String) {
    println!("{some_string}")
} // `some_string` goes out of scope & `drop` is called

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // `some_integer` goes out of scope

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string // some_string is returned and ownership moves out of the method
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
