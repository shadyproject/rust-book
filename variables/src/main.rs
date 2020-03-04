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

    let x = 2.1; //f64 by default
    let y: f32 = 3.2;

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    //addition
    let x = 5 + 10;
    println!("The value of x is {}", x);

    //subtraction
    let x = 10 - 5;
    println!("The value of x is {}", x);

    //multiplication
    let x = 4 * 30;
    println!("The value of x is {}", x);

    //division
    let x = 10 / 2;
    println!("The value of x is {}", x);

    //remainder
    let x = 43 % 5;
    println!("The value of x is {}", x);

    // booleans
    let t = true;
    let f: bool = false;

    if t {
        println!("True!");
    }

    if !f {
        println!("False!");
    }

    // characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("The value of c is {}", c);
    println!("The value of z is {}", z);
    println!("The value of heart_eyed_cat is {}", heart_eyed_cat);

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is {}", five_hundred);
    println!("The value of six_point_four is {}", six_point_four);
    println!("The value of one is {}", one);
}
