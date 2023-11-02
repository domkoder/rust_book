fn main() {
    // Varoable Scope
    {                           // s is not valide here, it's not yet declared
        let s = "hello";  // s is valid from thi poit forwards
        println!("{s}")
    }                           // this scope is now over, and is no longer valid

    // The String Type
    let mut s = String::from("hello");  // push_str() appends a literal to a String
    s.push_str(", world!");
    println!("{}", s);

    // Variable and Data Interacting with Move
    let x: i32 = 5;
    let y: i32 = x;

    let s1: String = String::from("hello");
    let s2: String = s1;

    // println!("{}, world", s1);

    // Variables and Data Interacting with Clone
    let s3: String = String::from("hello");
    let s4: String = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    // Ownership and Functions
    let sf1: String = gives_ownership(); // gives_ownership moves its return value into sf1

    let sf2: String = String::from("hello"); // sf2 comes into scope

    let sf3: String = takes_and_gives_back(sf2); // sf2 is move into takes_and_gives_back, which also moves its return value into sf3

} // Here, sf3 goes out of scope and is dropped. sf2 was moved  so nothing happens. sf1 goes out of scope and is dropped.


fn gives_ownership() -> String { // gives_ownership will move its return value in to the function that calls it
    let some_string: String = String::from("yours"); // some_string comes into scope
    some_string // some_string is return and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String{ // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}