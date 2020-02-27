fn main() {
    /*
    //mutable variable
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("THe value of x is {}", x);
     */

    //variable shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is {}", x);
}
