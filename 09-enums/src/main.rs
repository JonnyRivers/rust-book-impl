enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    {
        let home = IpAddr::V4(String::from("127.0.0.1"));

        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        // Option<T> is like Nullable<T> in C#
        let some_number = Some(5);
        let some_string = Some("a string");

        let absent_number: Option<i32> = None;
    }

    {
        let nickel = Coin::Nickel;
        let nickel_value = value_in_cents(nickel);
        println!("Nickel is worth {} cents", nickel_value);
    }

    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        println!("five: {}; six: {}; none: {}", five.unwrap_or_default(), six.unwrap_or_default(), none.unwrap_or_default());
    }

    {
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }

        // if let is more concise
        let some_u8_value = Some(0u8);
        if let Some(3) = some_u8_value {
            println!("three");
    }
    }
}
