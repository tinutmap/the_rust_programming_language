fn main() {
    // If
    let x = 3;

    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is less than or equal 5");
    }

    let number = 7;
    if number != 0 {
        println!("number is not 0");
    }

    if number % 4 == 0 {
        println!("number is divisible to 4");
    } else if number % 3 == 0 {
        println!("number is divisible to 3");
    } else if number % 2 == 0 {
        println!("number is divisible to 2");
    } else {
        println!("number is not divisible to 4,3,2");
    }

    let condition = true;
    let y = if condition { 5 } else { 6 };
    // let y = if condition { 5 } else { "six" }; // NOTE: will fail due to type mismatch.
    println!("y={y}");

    // Loop
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("counter={result}");

    let mut count = 0;
    'count_up: loop {
        println!("count= {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'count_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut n = 3;
    while n != 0 {
        println!("n={n}");
        n -= 1;
    }
    println!("LIFTOFF!!!");

    // For loop
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("a[{index}]={}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value of element is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}")
    }
    println!("LIFTOFF!!!");
}
