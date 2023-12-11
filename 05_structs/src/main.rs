
#[derive(Debug)]
struct  Rectangel {
    width: u32,
    height: u32,
}

impl Rectangel {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1: Rectangel = Rectangel {
        width: 30,
        height: 50,
    };
    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    if rect1.width() {
    println!("The rectangle has a nonzero width; it is {}", rect1.width)
    };

}

