
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

    fn max(self, other: Rectangel)-> Rectangel {
        Rectangel {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangel) {
        *self = self.max(other)
    }
}

// impl Rectangel {
//     fn can_hold(&self, other:&Rectangel)-> bool{
//         self.width > other.width && self.height > other.height 
//     }

//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size
//         }
//     }
// }

fn main() {
    // let rect1: Rectangel = Rectangel {
    //     width: 30,
    //     height: 50,
    // };

    // let rect2: Rectangel = Rectangel {
    //     width: 10,
    //     height: 40,
    // };

    // let rect3: Rectangel = Rectangel {
    //     width: 60,
    //     height: 45
    // };

    // let mut r: Rectangel = Rectangel{
    //     width: 1,
    //     height: 2
    // };

    // let area1: u32 = r.area();
    // let area2: u32 = Rectangel::area(&r);
    // assert_eq!(area1, area2);

    // r.set_width(2);
    // Rectangel::set_width(&mut r, 2);

    // Rectangel::square(20);


    // dbg!(&rect1);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );

    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect3));

    let  rect: Rectangel = Rectangel {
        width: 0,
        height:0
    };

    let other_rect: Rectangel = Rectangel {
        width:1, 
        height:1
    };

    let max_rect: Rectangel = rect.max(other_rect);

    println!("{}", rect.area())
}

