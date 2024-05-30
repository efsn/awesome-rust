#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    let rec1 = Rectangle {
        width: 35,
        height: 45,
    };

    let rec2 = Rectangle::square(25);

    println!("Can rec hold rec1? {}", rec.can_hold(&rec1));
    println!("Can rec hold rec2? {}", rec.can_hold(&rec2));

    println!("rec is {:#?}", &rec); // print to stdout
    dbg!(&rec); // print to stderr
    println!("The area of the rectangle is {} square pixels.", rec.area());
}
