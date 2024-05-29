#[derive(Debug)]
pub enum Value {
    Int(i32, i32),
    Float(f32),
    UnsignedInt(u32)
}

pub fn min(arr: Vec<i32>) -> Option<i32> {
    let mut smallest = None;
    for num in arr {
        if num < previous {
            smallest = Some(num);
        }
    }
    smallest
}

fn main() {
    let int = Value::Int(5, 12);
    let float = Value::Float(6.3);
    let unsigned_int = Value::UnsignedInt(7);

    let some = Some(5);
    let none: Option<i32> = None;
}
