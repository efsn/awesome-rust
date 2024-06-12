mod spread_sheet;
mod vector_loop;

fn main() {
    let v = Vec::<i32>::new();

    let _v1 = vec![1, 2, 3];

    let mut v2 = Vec::new();
    v2.push("value");

    let v3 = vec![1, 2, 3, 4, 5];
    let thd = &v3[2];
    println!("The third element is {}", thd);

    match v.get(2) {
        Some(thd) => println!("The third element is {}", thd),
        None => println!("There's no third element"),
    }

    vector_loop::l_o_o_p();

    println!("Hello, world!");
}
