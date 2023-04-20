fn main() {
    let mut s = String::from("hello world");

    let first_word_index = first_word(&s);
    println!("{first_word_index}");

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{hello} {world}");

    let whole_s_slice = &s[..];
    println!("{whole_s_slice}");

    let first_word = first_word_slice(&s);
    // s.clear(); // this will cause error as .clear() is mutable borrow whereas first_word is a immutable borrow that still valid at println!()
    println!("first word = {first_word}");

    let my_string = String::from("hello world");

    let word = first_word_slice(&my_string[..6]);
    println!("{word}");
    let word = first_word_slice(&s[..]);
    println!("{word}");
    let word = first_word_slice(&my_string);
    println!("{word}");

    let string_literal = "hello world";
    let word = first_word_slice(&string_literal[..6]);
    println!("{word}");
    let word = first_word_slice(&string_literal[..]);
    println!("{word}");
    let word = first_word_slice(string_literal);
    println!("{word}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[..2];
    assert_eq!(slice, [1, 2]); // Passed
    assert_eq!(slice, [1, 2, 3]); // Failed
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return index;
        }
    }
    s.len()
}

// fn first_word_slice(s: &String) -> &str {
fn first_word_slice(s: &str) -> &str {
    // this is a better function signature instead of s: &String
    let bytes = s.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..index];
        }
    }
    &s[..]
}
