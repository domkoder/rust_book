struct Point {
	x: f64,
	y: f64
}

impl Point {
    fn origin() -> Point {
        Point {x: 0.0, y: 0.0}
    }

    fn new(x:f64, y: f64) -> Point {
		Point{x: x, y: y}
    }
}

struct  Rectangle {
	p1: Point,
	p2: Point
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 -y2).abs())
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {    
  fn max(self, other: Self) -> Self {
    let w = self.width.max(other.width);
    let h = self.height.max(other.height);
    Rectangle { 
      width: w,
      height: h
    }
  }
    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);
        *self = max;
    }
}

fn main() {
    let mut rect = Rectangle { width: 0, height: 1 };
    let other_rect = Rectangle { width: 1, height: 0 };
    rect.set_to_max(other_rect);
}