fn main() {
    let mut s = String::from("hello world");

    let word = first_word_no_slice(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // slices
    {
        let s = String::from("hello world");

        let len = s.len();

        let hello: &str = &s[0..5]; // ending index is exclusive
                                    // let hello: &str = &s[..5]; // also valid, 0 is implied
        let world: &str = &s[6..11];
        // let world: &str = &s[6..]; // also valid, the ending index is implied based off of `s`
        let hello_world = &[0..len];
        let hello_world = &[..]; // also valid, a slice of the entire
        let s2 = &String = &s; // not a slice
    }

    // first example now would throw an error
    {
        let mut s = String::from("hello world");

        let word = first_word_with_slice(&s); // immutable reference

        s.clear(); // error, mutable reference needed

        println!("the first word is: {}", word);
    }

    {
        let my_string = String::from("hello world");

        // `first_word` works on slices of `String`s, whether partial or whole
        let word = first_word_with_slice(&my_string[0..6]);
        let word = first_word_with_slice(&my_string[..]);
        // `first_word` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let word = first_word_with_slice(&my_string);

        let my_string_literal = "hello world";

        // `first_word` works on slices of literals, whether partial or whoel
        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);
    }
}

fn first_word_no_slice(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // b' ' byte literal syntax
            return i;
        }
    }

    s.len()
}

// &str
fn first_word_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
