use std::io;
fn main() {
    // Scalar types

    // Decimal.
    // NOTE: `_` can be used to make the number easier to read.
    let dec = 98_222;
    println!("dec = {dec}");

    let hex = 0xff;
    println!("hex = {hex}");

    let octal = 0o77;
    println!("octal = {octal}");

    let binary = 0b1111_0000;
    println!("binary = {binary}");

    let byte = b'A';
    println!("byte = {byte}");

    // Floating point types
    let x_f64 = 2.0;
    let y: f32 = 3.0;
    println!("x_f64 = {x_f64}, y = {y}");

    let sum = 5 + 10;
    println!("sum = {sum}");

    let difference = 10.2 - 6.2;
    println!("difference = {difference}");

    let quotient = 56.7 / 32.2;
    println!("quotient = {quotient}");

    let truncated = 7 / 3; // NOTE: truncated since integer / integer
    println!("truncated = {truncated}");

    // Boolean
    let tr = true;
    let fa: bool = false;
    println!("tr={tr}, fa={fa}");

    // Character type
    // NOTE: char is single quote ''
    let ch = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("ch= {ch}, z = {z}, heart_eyed_cat= {heart_eyed_cat}");

    // Compound Types
    // Tuple Type
    let tup = (500, 6.4, 'a');
    // println!("tup={tup}"); // does not work

    let (int, flo, cha) = tup; // NOTE: tuple destructuring;
    println!("int={int}, flo={flo}, cha={cha}");

    // NOTE: access individual element
    let int = tup.0;
    let flo = tup.1;
    let cha = tup.2;
    println!("int={int}, flo={flo}, cha={cha}");

    // Array Type
    let arr_num = [1, 2, 3, 4, 5];
    let arr_same_num = [0; 5];
    // println!("arr_num={arr_num}, arr_same_num={arr_same_num}"); // does not work

    let arr_num_0 = arr_num[0];
    let arr_same_num_3 = arr_same_num[3];
    println!("arr_num_0={arr_num_0}, arr_same_num_3={arr_same_num_3}");

    // Invalid array access out of index
    let mut index = String::new();

    println!("Please input array index: ");
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered not a integer");

    let element = arr_num[index];
    println!("The value of index {index} in arr_num is: {element}");
}
