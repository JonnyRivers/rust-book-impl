fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, _z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let _five_hundred = tup.0;

    let _six_point_four = tup.1;

    let _one = tup.2;

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a[2] is: {}", a[2]);

    let b = ["Jonny"; 7];
    println!("The value of b[5] is: {}", b[5]);
}
