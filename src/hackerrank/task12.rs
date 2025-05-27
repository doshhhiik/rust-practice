use std::io;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

fn between_two_sets(a: &[i32], b: &[i32]) -> usize {
    let lcm_a = a.iter().fold(1, |acc, &x| lcm(acc, x));
    let gcd_b = b.iter().fold(b[0], |acc, &x| gcd(acc, x));

    (lcm_a..=gcd_b).step_by(lcm_a).filter(|&x| gcd_b % x == 0).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    println!("{}", between_two_sets(&a, &b));
}
