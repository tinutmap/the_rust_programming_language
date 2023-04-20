fn main() {
    println!("Hello, world!");

    // another_fn();
    another_fn(3);
    print_label_measurement(4, 'm');

    // Statements and expressions
    let y = {
        let x = 3;
        x + 1
    };
    println!("y={y}"); // =4

    let five = five();
    println!("five={five}");

    let x_plus_1 = plus_one(5);
    println!("x_plus_1={x_plus_1}");
}

// fn another_fn() {
//     println!("From another fn")
// }
fn another_fn(x: i32) {
    println!("x = {x}")
}

fn print_label_measurement(x: i32, unit: char) {
    println!("The measurement is: {x}{unit}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
