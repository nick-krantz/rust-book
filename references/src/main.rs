fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // pass a reference to s1,
                                     // calculate_length doesn't take ownership over s1

    // s1 is still valid in scope here
    println!("The length of '{s1}' is {len}.");

    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        // let r2 = &mut s; cannot borrow a mutable reference more than once

        println!("{r1}");
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but nothing magical with `s` as we never had ownership
