pub(crate) fn main() {
    println!("stdout below comes from interactive module");

    // ----------- < Variables Live in the Stack > -----------
    let a = 5;
    let mut b = a; // b is cloned from a
    b += 1;
    println!("a={}, b={}", a, b); // a=5, b=6

    let mut a = 5;
    let b = a; // b is cloned from a
    a += 1;
    println!("a={}, b={}", a, b); // a=6, b=5

    // ----------- <Boxes Live in the Heap > -----------
    let a = [0; 1_000];
    let b = a;
    println!("{:?}", b);

    // ----------- <Collections Use Boxes> -----------
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
    // println!("{first}"); // Will not work as first lost ownership in fn add_suffix()

    // ----------- <Cloning Avoids Moves> -----------
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");

    // ----------- <Quiz> -----------
    let s = String::from("hello");
    let s2;
    let b = false;
    if b {
        s2 = s;
    }
    // println!("{}", s); // will fail

    let b = Box::new(0);
    let b2 = b;
    // println!("{}", b); // This is technically safe but still rejected by Rust.
    move_a_box(b2);

    // ----------- <Seft test, if a in stack, it does not lose ownership in print_var()> -----------

    let a = 6;
    let b = a;
    print_var(a);
    println!("{a},{b}");
}
fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn move_a_box(b: Box<i32>) {
    println!("{:?}", b);
}

fn print_var(var: i32) {
    println!("var = {var}");
}
