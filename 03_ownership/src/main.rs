fn main() {
    {                           // s is not valide here, it's not yet declared
        let s = "hello";  // s is valid from thi poit forwards

        println!("{s}")
    }                           // this scope is now over, and is no longer valid

    let mut s = String::from("hello");  // push_str() appends a literal to a String
    
    s.push_str(", world!");

    println!("{}", s);

}
