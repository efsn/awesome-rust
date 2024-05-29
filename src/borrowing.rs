mod algebreaic;
use algebreaic::Value;
use std::fmt;

fn print(value: &Value) -> String {
    match value {
        Value::Int(number, 2) => format!("{:b}", number),
        Value::Int(number, 8) => format!("{:o}", number),
        Value::Int(number, 16) => format!("{:x}", number),
        Value::Int(number, _) => format!("{}", number),
        Value::Int(number) => format!("{:b}", number),
    }
}

fn mutate(str: &String) {
    println!("{}", str);
    str.push_str("There")
}

fn main() {
    let value = Value::Int(5, 10);
    println!("{}", print(&value));
    println!("{}", print(&value));
    println!("{}", print(&Value::Int(5, 12)));
    println!("{}", print(&Value::Int(5, 2)));
    println!("{}", print(&Value::Float(5.5)));

    let mut s = String::from("Hello");
    s.push_str(" ");
    mutate(&s);
    println!("###{}###", s)
}