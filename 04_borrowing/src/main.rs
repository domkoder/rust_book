fn main() {
    let mut s1: String = String::from("hello");

    let len: usize = calculate_length(&s1);

    // Mutable References
    change(&mut s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s: String = String::from("Hello");
    {
        let r1: &mut String = &mut s; 
    }; //r1 goes out of scope here, so we can make a new reference with no problems.
    let r2: &mut String = &mut s;

    println!("{}", r2);

    let reference_to_nothing: String = no_dangle();

    // The slice Type
    let word_string = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];


    let word = first_word(&word_string);

    let array: [i32; 5] = [1,2,3,4,5];

    let slice = &array[1..3];

    assert_eq!(slice, &[2, 3]);

    println!("{word}");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String 
    s.len()
} // Here, s goes out of scope. But because it doees not have ownership of what it refers to, it is not dropped.


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle return a reference to a string

//     let s: String = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.

fn no_dangle() -> String {
    let s: String = String::from("Hellow");

    s
}

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}