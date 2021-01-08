fn main() {
    const MAX_POINTS: u32 = 100_00;
    println!("Total number of points available: {}", MAX_POINTS);

    // shadowing lets you change the type of the variable, unlike mut
    let spaces = "    ";
    let spaces = spaces.len();

    println!("There are {} spaces.", spaces);
}
