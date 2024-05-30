fn min(arr: Vec<i32>) -> i32 {
    let mut smallest = i32::MAX;
    for num in arr {
        if num < smallest {
            smallest = num;
        }
    }
    smallest
}

fn main() {
    println!("{}", min(vec![2,4,5,6,1]))
}
