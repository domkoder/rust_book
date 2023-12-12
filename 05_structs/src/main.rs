
#[derive(Debug)]
struct  Rectangel {
    width: u32,
    height: u32,
}

impl Rectangel {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

impl Rectangel {
    fn can_hold(&self, other:&Rectangel)-> bool{
        self.width > other.width && self.height > other.height 
    }
}

fn main() {
    let rect1: Rectangel = Rectangel {
        width: 30,
        height: 50,
    };

    let rect2: Rectangel = Rectangel {
        width: 10,
        height: 40,
    };

    let rect3: Rectangel = Rectangel {
        width: 60,
        height: 45
    };

    let mut r: Rectangel = Rectangel{
        width: 1,
        height: 2
    };

    let area1: u32 = r.area();
    let area2: u32 = Rectangel::area(&r);
    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangel::set_width(&mut r, 2);

    Rectangel::square(20);


    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect3));

}

