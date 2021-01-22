fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[0]);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("{}", months[11]);

    //init an array with the same thing
    let b = [0; 5];
    println!("{}", b[3]);
}
