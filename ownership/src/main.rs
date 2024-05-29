fn main() {
    function();
}

fn allocate() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn move_value() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);
}

fn clone_value() {
    let x = String::from("hello");
    let y = x.clone();
    println!("x = {}, y = {}", x, y);
}

fn function() {
    let s = String::from("hello"); //s entry into scope
    take_ownership(s); // value of s moves to fn
                       // s is invalide

    let x = 5; // x entry scope
    make_copy(x); //  value of x copy to fn
}

fn take_ownership(some_string: String) {
    // some_string entry into scope
    println!("{}", some_string);
} // some_string move out from scope and invoke `drop`
  // mem free

fn make_copy(some_integer: i32) {
    // some_integer entry into scope
    println!("{}", some_integer);
} // some_integer move out from scope and invoke `drop`

fn x() {
    let s1 = give_ownersihp(); // return to s1
    let s2 = String::from("hello"); // s2 into scope
    let s3 = take_and_give_back(s2); // s2 move to fn and return to s3
}

fn give_ownersihp() -> String {
    // give_ownership will move result to invocation
    let some_string = String::from("yours"); // some_string entry into scope
    some_string // return some_string and move to invocation
}

fn take_and_give_back(str: String) -> String {
    // str entry into scope
    str // return str and move to invocation
}
