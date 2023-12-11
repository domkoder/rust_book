
#[derive(Debug)]
struct  Rectangel {
    width: u32,
    height: u32,
}

fn main() {
    let scale: u32 = 2;
    let rect1: Rectangel = Rectangel {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    println!("rect1 is {:#?}", rect1);

}

fn area(rectangel: &Rectangel) -> u32 {
    rectangel.width * rectangel.height
}