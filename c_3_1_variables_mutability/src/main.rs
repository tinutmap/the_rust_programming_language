fn main() {
    // // 1st try
    // let mut x = 5;
    // println!("The value of x is {x}");
    // x = 6;
    // println!("The value of x is {x}");

    // // 2nd try
    // let x = 5;

    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is {x}");
    // }

    // println!("The value of x is: {x}")

    // 3rd try
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The number of space is {spaces}");

    // // Below will fail due to mismatch type of spaces when making it mutable.
    // let mut spaces = "     ";
    // spaces = spaces.len();
}
