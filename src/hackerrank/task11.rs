use std::io;

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> &'static str {
    if v1 > v2 && (x2 - x1) % (v1 - v2) == 0 {
        "YES"
    } else {
        "NO"
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", kangaroo(nums[0], nums[1], nums[2], nums[3]));
}
