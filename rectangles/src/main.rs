#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let scale = 2;

    let rec = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rec is {:#?}", &rec); // print to stdout
    dbg!(&rec); // print to stderr
    println!(
        "The area of the rectangle is {} square pixels.",
        rec.area()
    );
}
