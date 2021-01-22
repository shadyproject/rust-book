fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //println!("Tuple: {}", tup); //doens't work

    // destructuring
    let (e, y, z) = tup;
    println!("The value of y is: {}", y);

    // direct access
    let five_hundred = tup.0;
    println!("{}", five_hundred);
}
