use std::rc::Rc;

mod quiz;
fn main() {
    // ----------- <Fixing Ownership Errors > -----------

    // ----------- <Fixing an Unsafe Program: Returning a Reference to the Stack > -----------
    // see `fn return_a_string()` and its associated solutions

    // ----------- <Fixing an Unsafe Program: Not Enough Permissions> -----------
    // let name = vec![String::from("Ferris")];
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    // stringtify_name_with_title(&name);
    // stringtify_name_with_title_solution_1(&mut name);
    // stringtify_name_with_title_solution_2(mut name); //still fail
    let mut name_full = stringtify_name_with_title_solution_3(&name); // NOTE: was name_full's value implicitly deferenced from stringtify_name_with_title_solution_3()?
    println!("{first}");
    // name_full.push();
    name_full.insert_str(0, &String::from("a String"));
    println!("{name_full}");

    // ----------- <Fixing an Unsafe Program: Aliasing and Mutating a Data Structure> -----------
    // See add_big_strings() and its solutions

    // ----------- <Fixing an Unsafe Program: Copying vs. Moving Out of a Collection> -----------
    let v = vec![1, 2, 3];
    let n_ref = &v[0];
    let n = *n_ref; // this works but what happens when we turn to String, see below

    let v = vec![String::from("Hello World")];
    let s_ref = &v[0];
    // let s = *s_ref; // fail

    // it will fails due to the implicit drop() below
    // drop(s);
    // drop(v);

    // solution 1: just use the reference
    println!("{s_ref}!");

    // solution 2: clone
    let s = &mut v[0].clone();
    s.push('!');
    println!("{s}");

    // solution 3: remove string from vector
    let mut v = vec![String::from("Hello World")];
    let mut s = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.is_empty());

    // ----------- <Fixing a Safe Program: Mutating Different Tuple Fields> -----------
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first_name = &name.0;
    name.1.push_str(" Esp.");
    println!("{first_name}, {}", name.1); //is OK but if we do below

    let first_name = get_first(&name);
    // name.1.push_str(" Esp."); //is NOK here as name lost ownership by get_first()
    println!("{first_name}, {}", name.1);

    // ----------- <Fixing a Safe Program: Mutating Different Array Elements> -----------
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0];
    *x += 1;

    // let y = &a[1];
    // *x += *y;// will fail as `a` lost all R W O privileges;
    println!("{a:?}");

    // solution 1:
    let (x, rest) = a.split_first_mut().unwrap();
    let y = &rest[0];
    *x += *y;

    // solution 2 but make sure you know what you're doing
    let x = &mut a[0] as *mut i32;
    let y = &a[1] as *const i32;
    unsafe {
        *x += *y;
    } // DO NOT DO THIS unless you know what you're doing!

    // quiz::main();
}

// this fn() fails, see possible fixes below
// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s // IMO, fail here as s will be dropped at the end of function.
// }

fn _return_a_string_sol_1() -> String {
    let s = String::from("Hello world");
    s
}
fn _return_a_string_sol_2() -> &'static str {
    "Hello World"
}
fn _return_a_string_sol_3() -> Rc<String> {
    let s = Rc::new(String::from("Hello World"));
    Rc::clone(&s)
}
fn _return_a_string_sol_4(output: &mut String) {
    output.replace_range(.., "Hello World");
}

// // will fail as name is immutable
// fn stringtify_name_with_title(name: &Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

// // this solution is a potential way but still not feasible
// fn stringtify_name_with_title_solution_1(name: &mut Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

// // this solution is a potential way but still not feasible
// fn stringtify_name_with_title_solution_2(mut name: Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

fn stringtify_name_with_title_solution_3(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

// improvement based on stringtify_name_with_title_solution_3
fn stringtify_name_with_title_solution_4(name: &Vec<String>) -> String {
    let mut name_clone = name.join(" ");
    name_clone.push_str("Esq.");
    name_clone
}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest = dst.iter().max_by_key(|s| s.len()).unwrap();

    for s in src {
        if s.len() > largest.len() {
            // dst.push((s.clone())); // fail here as dst lost ownership to largest.
        }
    }
}

// possible solution, not very good as it is costly to clone dst.
fn add_big_strings_solution_1(dst: &mut Vec<String>, src: &[String]) {
    let largest = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push((s.clone())); // fail here as dst lost ownership to largest.
        }
    }
}

// possible solution, adding the remaining to the end
fn add_big_strings_solution_2(dst: &mut Vec<String>, src: &[String]) {
    let largest = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> = src
        .iter()
        .filter(|s| s.len() > largest.len())
        .cloned()
        .collect();
    dst.extend(to_add);
}

// best and most idiomatic way
fn add_big_strings_solution_3(dst: &mut Vec<String>, src: &[String]) {
    let largest_len = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

fn get_first(name: &(String, String)) -> &String {
    &name.0
}
