struct Shape {
    width: u32,
    height: u32,
}

impl Shape {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    fn square(size: u32) -> Self {
        Shape {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Shape {
        width: 10,
        height: 20,
    };

    let square1 = Shape::square(15);

    println!("Rectangle area: {}", rect.area());
    println!("Rectangle perimeter: {}", rect.perimeter());
    println!("Square area: {}", square1.area());
    println!("Square perimeter: {}", square1.perimeter());

    println!("Is rectangle a square? {}", rect.is_square());
    println!("Is square a square? {}", square1.is_square());

    let rect2 = Shape {
        width: 20,
        ..rect
    };

    println!("New rectangle area: {}", rect2.width);
    println!("New rectangle area: {}", rect2.height);
    println!("New rectangle area: {}", rect.height);
    println!("New rectangle area: {}", rect.width);



}
