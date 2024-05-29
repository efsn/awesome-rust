

#[derive(Debug)]
struct ConversionError;

impl Error for ConversionError {}

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Some error")
    }
}

fn format(value: Value) -> Result<String, ConversionError> {
    match value {
        Value::Int(number, 2) => Ok(format!("{:b}", number)),
        Value::Int(number, 8) => Ok(format!("{:o}", number)),
        Value::Int(number, 16) => Ok(format!("{:x}", number)),
        Value::Int(_, _) => Err(ConversionError),
        Value::Float(number) => Ok(format!("{}", number)),
    }
}

fn main() {
    let result = format(Value::Int(5,6))
        .map(|f|format!("Formatted Value {}", f))
        .map_err(|err|format!("There was an error {}", err));
    println!(":?", result);
}
