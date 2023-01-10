/* structs */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// all functions within an impl block are called "associated" functions
impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }

    // this longhand notation is Ok also for borrowing the Self instance
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    // ERROR: functions must have a `self` parameter
    // fn area(r: &Rectangle) -> u32 {
    //     r.width * r.height
    // }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // build a square
    fn gen_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30u32;
    let height1 = 50u32;
    println!("The area is {} pixels", area(width1, height1));

    let rect1 = (30u32, 50u32);
    println!("The area is {} pixels", area2(rect1));

    let rect2 = Rectangle {
        width: 30u32,
        height: 50u32,
    };
    println!("The area of rect2 is {} pixels", area3(&rect2));
    println!("The printing of rect2 is {:?}", rect2);

    let rect3 = Rectangle {
        width: 30u32,
        height: 50u32,
    };
    println!("The area of rect3 is {}", rect3.area());

    println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));

    let my_square = Rectangle::gen_square(30u32);
    println!("The area of my_square is {}", my_square.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
