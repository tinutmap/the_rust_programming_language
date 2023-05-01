mod interactive;
mod quiz;

fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("{s1} has length of {len}");

    let mut s = String::from("Hello");

    change(&mut s);
    println!("s={s}");

    let r1 = &mut s;
    // let r2 = &mut s; // NOTE: this will fail as only one mut reference/borrow allowed.
    // println!("{r1}, {r2}");

    // NOTE: above can be fixed with using different scope.
    {
        let r2 = &mut s;
    } // r2 goes out of scope here, so we can make a new reference with no problems.

    // let r3 = &s; // no problem
    // let r4 = &s; // no problem
    // let r5 = &mut s; // BIG problem: Users of an immutable reference don’t expect the value to suddenly change out from under them!
    // println!("{r3}, {r4}, {r5}");

    // code above can be fixed by
    let r3 = &s;
    let r4 = &s;
    println!("r3={r3}, r4={r4}");
    // variables r3 and r4 will not be used after this point

    let r5 = &mut s; // therefore no problem r5 being mut ref here.
    println!("r5={r5}");

    // Dangling reference
    // let reference_to_nothing = dangle(); // NOTE: this will fail as trying to create a dangling pointer

    let no_dangle_reference = no_dangle();
    println!("{no_dangle_reference}");

    // ----------- <from Interactive module > -----------
    interactive::main();
    quiz::main();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(s: &mut String) {
    s.push_str(", world!");
}

// fn dangle()-> &String{ // dangle returns a reference to a String
//     let s = String::from("Hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//  // Danger!

fn no_dangle() -> String {
    // let s = String::from("hello");
    // s
    String::from("hello")
}
