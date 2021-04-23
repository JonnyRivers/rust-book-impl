fn main() {
    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];

        println!("{} {}", hello, world);

        // string slices are a pointer and a length
        let _slice = &s[..2];
        let _slice = &s[7..];
    }

    // a string with outstanding slices *cannot* be modified
    {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        // mutable borrow is not allowed!!!
        s.clear(); // error!

        println!("the first word is: {}", word);
    }

    {
        let my_string = String::from("hello world");

        // first_word works on slices of `String`s
        let _word = first_word(&my_string[..]);

        let my_string_literal = "hello world";

        // first_word works on slices of string literals
        let _word = first_word(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let _word = first_word(my_string_literal);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}