use std::fmt::Debug;
use std::fmt;
use std::fmt::Formatter;


// Traits are similar to Java interfaces
// Don't have known size at compile time
// Traits can be inherited

impl Debug for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "I'm a rectangle with length {} and breadth {}", self.length, self.breadth)
    }
}

pub trait Shape {
    fn area(&self) -> i32;
}

pub trait DebuggableShape: Shape + Debug {}

impl Shape for Rectangle {
    fn area(&self) -> i32 {
        self.length * self.breadth
    }
}

impl DebuggableShape for Rectangle {}

fn main() {
    let rectangle = Rectangle::new(4,5);
    let square = Square::new(5);
    let shape: &dyn Shape = &square;

    println!("Rect: {:?}", rectangle);
    println!("{Square: {:?}", square);
    println!("Shape: {:?}", shape.area());
    
    let debuggable_shape: &dyn DebuggableShape = &rectangle;
    
    println!(":?", debuggable_shape);
    println!(":?", debuggable_shape.area());
}
