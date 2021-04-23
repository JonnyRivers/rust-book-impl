fn main() {
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let mut s = String::from("hello");

        change(&mut s);
    }

    {
        let mut s = String::from("hello");

        let _r1 = &mut s;
        // cannot take a second mutable borrow
        //let r2 = &mut s;
    }

    {
        let mut s = String::from("hello");

        let _r1 = &s; // no problem
        let _r2 = &s; // no problem
        // cannot borrow s as mutable because immutable borrows exist
        //let r3 = &mut s; // BIG PROBLEM
    }

    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // r1 and r2 are no longer used after this point

        let r3 = &mut s; // no problem
        println!("{}", r3);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}