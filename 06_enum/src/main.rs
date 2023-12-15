#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quite,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call (&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let home: IpAddr = IpAddr::V4(127, 0, 0, 1);

    let loopback: IpAddr = IpAddr::V6(String::from("::1"));

    println!("{:#?} {:#?}", home, loopback);

    let m = Message::Write(String::from("Hellow"));
    m.call()
}
