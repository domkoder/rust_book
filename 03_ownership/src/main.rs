fn main() {
    {                           // s is not valide here, it's not yet declared
        let s = "hello";  // s is valid from thi poit forwards

        println!("{s}")
    }                           // this scope is now over, and is no longer valid

}
