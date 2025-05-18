fn a_very_big_sum(arr: &[i64]) -> i64 {
    arr.iter().sum()
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // читаємо кількість елементів, але вона нам не потрібна
    
    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    println!("{}", a_very_big_sum(&numbers));
}
