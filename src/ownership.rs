mod algebreaic;
mod pattern_match;
use algebreaic::Value;

fn main() {
    let value = Value::Int(5, 10);
    println!("{}", print(value));
    println!("{}", print(Value::Int(5, 11)));
    println!("{}", print(Value::Int(5, 2)));
    println!("{}", print(Value::Float(5.5)));
}
