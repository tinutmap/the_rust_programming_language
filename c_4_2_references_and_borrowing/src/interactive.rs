pub fn main() {
    // ----------- < References and Borrowing > -----------

    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);
    // let s = format!("{}, {}", m1, m2); // will fail as m1, m2 was freed

    let m1 = String::from("Hello");
    let m2 = String::from("world");
    // NOTE: alternatively, as a inconvenient way to return m1, m2
    let (m1_again, m2_again) = greet_return(m1, m2);
    let _s = format!("{}, {}", m1_again, m2_again); // will fail as m1, m2 was freed

    // ----------- < References Are Non-Owning Pointers > -----------
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet_reference(&m1, &m2);
    let _s = format!("{}, {}", m1, m2);

    // ----------- <Dereferencing a Pointer Accesses Its Data > -----------
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;
    let mut b = *x;
    *x += 1;
    println!("a={},*x = {}, b = {}", a, *x, b);

    let r1: &Box<i32> = &x;
    let c: i32 = **r1;
    println!("c = {}", c);

    let r2: &i32 = &*x;
    let d = *r2;
    println!("d = {}", d);

    let mut r3 = x; // r3 takes over x's ownership from here
    let e = *r3;
    println!("e = {}", e);
    *r3 += 1;
    println!(
        "a={}, b = {}, c = {}, d = {},*r3 = {} e = {}",
        a, b, c, d, *r3, e
    );

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice).NOTE: though you can't write **r.abs(), see below for self test.
    assert_eq!(r_abs1, r_abs2);
    let r_deference_twice = **r;
    let r_deference_twice_absolute = r_deference_twice.abs();
    println!(
        "r_deference_twice = {},r_deference_twice_absolute={} ",
        r_deference_twice, r_deference_twice_absolute
    );

    // ----------- <Rust Avoids Simultaneous Aliasing and Mutation > -----------
    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);

    let mut vec2 = vec![1, 2, 3];
    let num = &vec2[2];
    vec2.push(4);
    // println!("num={}", *num); // will fail as vec2.push will deallocate after copy data to new memory address causing num pointing to invalid memory address.

    // ----------- <References Change Permissions on Paths > -----------
    let mut vec3 = vec![1, 2, 3];
    let num = &vec3[2];
    // let mut num_mut = &vec2[1];
    // *num_mut = 4; // failed here *num_mut doesn't have write privilege. see below.
    println!("num={}", *num);
    // println!("num={},num_mut={}", num, num_mut);

    vec3.push(4);
    println!("vec3= {:?}", vec3);

    let x = 0;
    let mut _x_ref = &x;
    // *_x_ref += 1; // *_x_ref does not have Write privilege here.

    let mut vec3 = vec![1, 2, 3];
    let num = &vec3[2];
    println!("num={}", *num);
    println!("num again={}", *num);
    vec3.push(4);
    println!("vec3= {:?}", vec3);

    // ----------- <The Borrow Checker Finds Permission Violations > -----------
    let mut vec4 = vec![1, 2, 3];
    let num = &vec4[2];
    vec4.push(4);
    // println!("*num ={:?}", *num)

    // ----------- <Mutable References Provide Unique and Non-Owning Access to Data > -----------
    let mut vec5 = vec![1, 2, 3];
    let num = &mut vec5[2];

    *num += 1;
    // println!("vec5={:?}", vec5); // will fail here as vec5 has not got Read privilege yet.
    println!("*num ={:?}", *num);
    println!("vec5={:?}", vec5);

    let mut vec6 = vec![1, 2, 3];
    let num = &mut vec6[2];
    let num2 = &*num; // num2 downgraded or overruled num here making *num losing Write privilege
    println!("*num={},*num2={}", *num, *num2);

    // trial with let `num2 =  &mut *num`
    let mut vec6 = vec![1, 2, 3];
    let num = &mut vec6[2];
    let num2 = &mut *num; // num2 downgraded or overrule num here making *num losing Write privilege

    *num2 += 1;
    // println!("*num={},*num2={}", *num, *num2); // will fail as num2 also takes *num1 Read privilege with &mut
    println!("*num2={}", *num2); // Rather, this will make *num2 forfeits all privileges.
    *num += 1;
    println!("*num={}", *num); // Making *num having all rights here.
    println!("vec6={:?}", vec6);

    // ----------- <Permissions Are Returned At The End of a Reference's Lifetime > -----------
    let mut x = 1;
    let y = &x; // y lifetime starts here
                // x = 2; // will fail
    let z = *y; // y lifetime ends here
    x = 2; //will be OK

    let mut chars = vec!['a', 'b', 'c'];
    ascii_capitalize(&mut chars);
    println!("chars = {:?}", chars);

    // ----------- <Data Must Outlive All Of Its References > -----------
    let s = String::from("Hello World");
    let s_ref = &s;

    // drop(s); // fail here as s does not outlive s_ref in the next line;
    println!("{s_ref}");

    let strings: Vec<String> = vec![];
    let default = String::from("default");
    // let s = first_or(&strings, &default) // comment so module does not fail due to first_or()
    drop(default);
    // println!("{s}");// comment so module does not fail due to first_or()
}

fn greet(m1: String, m2: String) {
    println!("{m1}, {m2}");
}

fn greet_return(m1: String, m2: String) -> (String, String) {
    println!("{m1}, {m2}");
    (m1, m2)
}

fn greet_reference(m1: &String, m2: &String) {
    println!("{m1}, {m2}");
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}
fn firtst(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0];
    return s_ref;
}

// // fail here as missing lifetime specifier
// fn first_or(strings: &Vec<String>, default: &String) -> &String {
//     if strings.len() > 0 {
//         &strings[0]
//     } else {
//         default
//     }
// }

// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     let s_ref = &s; // from my own opinion, this fails due to s will be dropped when this function returns s_ref;
//     s_ref
// }
