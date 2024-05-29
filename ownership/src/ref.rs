fn calc_len(s: &String) -> usize {
    s.len()
}

fn main() {
    let s1 = String::from("hello");
    let len = calc_len(&s1);
    println!("the length of '{}' is {}.", s1, len);
}


