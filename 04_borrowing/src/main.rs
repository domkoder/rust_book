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