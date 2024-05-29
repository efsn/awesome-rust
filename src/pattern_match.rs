#[derive(Debug)]
pub enum Value {
    Int(i32, u32),
    Float(f32),
}

pub fn print(value: Value) -> String {
    match value {
        Value::Int(5, 2) => format!("101"),
        Value::Int(number, 2) => format!("{:b}", number),
        Value::Int(number, 8) => format!("{:o}", number),
        Value::Int(number, 16) => format!("{:x}", number),
        Value::Int(_, radix) if radix != 10 => format!("Bad radix {}", radix),
        Value::Int(number, _) => format!("{:x}", number),
        Value::Float(number) => format!("{:}", number),
    }
}

fn main() {
    println!("{}", print(Value::Int(5, 10)));
    println!("{}", print(Value::Int(5, 11)));
    println!("{}", print(Value::Int(5, 2)));
    println!("{}", print(Value::Float(5.5)));
}
