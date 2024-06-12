#[derive(Debug)]
#[warn(dead_code)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn x() {
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.33),
        SpreadsheetCell::Text(String::from("green")),
    ];
}
