#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    let rect1 = (50, 30);
    let rect2 = Rectangle { length: 50, width: 30 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("rect2 is {:#?}", rect2);

    println!(
        "The area of the rectangle is {}, {}, {} square pixels.",
        area(rect1),
        area2(&rect2),
        rect2.area()
    );

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
}

fn area(dimensions : (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rect : &Rectangle) -> u32 {
    rect.length * rect.width
}