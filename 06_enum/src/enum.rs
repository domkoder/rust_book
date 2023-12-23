
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x:i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}


impl Message {
    fn call(&self) {
        println!("{:?}", &self)
    }
}
fn main() {

}
