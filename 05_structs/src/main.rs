struct  Rectangel {
    width: u32,
    height: u32,
}

fn main() {
    let rect1: Rectangel = Rectangel {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    )

}

fn area(rectangel: &Rectangel) -> u32 {
    rectangel.width * rectangel.height
}