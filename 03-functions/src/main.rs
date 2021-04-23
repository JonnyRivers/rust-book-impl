fn main() {
    println!("Hello, world!");

    a_function();

    another_function(3, 5);

    let a = 5;

    let b = {
        let a = 3;
        a + 1
    };

    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);

    let z = five();

    println!("The value of z is: {}", z);

    let w = plus_one(5);

    println!("The value of w is: {}", w);
}

fn a_function() {
    println!("Another function.");
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}