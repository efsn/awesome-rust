

pub struct Rectangle {
    length: i32,
    breadth: i32,
}

impl Rectangle {
    pub fn new(length: i32, breadth: i32) -> Rectangle {
        Rectangle {length, breadth}
    }

    fn area(&self) -> i32 {
        self.length * self.breadth
    }
}