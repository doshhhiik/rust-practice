use std::io;

fn plus_minus(arr: &[i32]) {
    let total = arr.len() as f64;
    let positive_count = arr.iter().filter(|&&x| x > 0).count() as f64;
    let negative_count = arr.iter().filter(|&&x| x < 0).count() as f64;
    let zero_count = arr.iter().filter(|&&x| x == 0).count() as f64;

    println!("{:.6}", positive_count / total);
    println!("{:.6}", negative_count / total);
    println!("{:.6}", zero_count / total);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    let mut arr = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    arr = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}
