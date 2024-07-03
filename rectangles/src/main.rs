
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 100
    };

    let rect2 = Rectangle {
        width: 90,
        height: 30
    };

    let rect3 = Rectangle {
        width: 101,
        height: 99
    };

    println!("The area of the rectangle {rect1:?} is {} square pixels.", rect1.area());
    println!("The area of the rectangle {rect2:?} is {} square pixels.", rect2.area());
    println!("The area of the rectangle {rect3:?} is {} square pixels.", rect3.area());

    println!("Can {rect1:?} hold {rect2:?} ? {}", rect1.can_hold(&rect2));
    println!("Can {rect1:?} hold {rect3:?} ? {}", rect1.can_hold(&rect3));
    println!("Can {rect2:?} hold {rect3:?} ? {}", rect2.can_hold(&rect3));
}
