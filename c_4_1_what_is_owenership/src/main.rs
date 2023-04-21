mod interactive;
fn main() {
    let mut s = String::from("Hello");

    s.push_str(", world!");
    println!("{s}");

    let s1 = String::from("hello");

    // NOTE: will not work due to ownership rule
    // let s2 = s1;
    // println!("s1={s1}");

    let s2 = s1.clone();
    println!("s1={s1}, s2={s2}");

    // Ownership and Functions
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here. therefore println!() below will fail
                        // println!("{s}");

    let x = 5;
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward such as in println!() below
    println!("x={x}");

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    println!("{s1}");

    let s2 = String::from("hello");

    let s3 = takes_and_give_back(s2); // s2 lose ownership from here so println!() below would fail
                                      // println!("s2 = {s2}");
    println!("s3={s3}");

    let (s4, len) = calculate_length(s1);
    println!("{s4} has length of {len}");

    interactive::main();
}

fn takes_ownership(s: String) {
    println!("takes_ownership s={s}");
}

fn makes_copy(x: i32) {
    println!("makes_copy x={x}");
}

fn gives_ownership() -> String {
    // let s = String::from("hello");
    // s
    String::from("hello") //NOTE: this is different, probably an improvement from the book code in the 2 previous lines that were commented out.
}

fn takes_and_give_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
