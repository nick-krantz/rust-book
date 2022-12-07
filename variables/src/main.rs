fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
        // inner shadow of `y` ends here to outer scope `y` will be available
    }

    println!("The value of y is: {y}");

    let spaces = "    "; // str type
    let spaces = spaces.len(); // usize type

    // let mut spaces = "    ";
    // spaces = spaces.len(); compile error because we're change the type on mutable variable

    let z = 2.0;

    let a: f32 = 3.0;

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // deconstruct the tuple
    let (c, d, e) = tup;
}
