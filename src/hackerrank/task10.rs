use std::io;

fn count_fruits(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter().filter(|&&d| a + d >= s && a + d <= t).count();
    let orange_count = oranges.iter().filter(|&&d| b + d >= s && b + d <= t).count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (s, t) = (parts[0], parts[1]);

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (a, b) = (parts[0], parts[1]);

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let _m: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let apples: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let oranges: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    count_fruits(s, t, a, b, &apples, &oranges);
}
