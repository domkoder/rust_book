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
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world", s1);
}
