use std::io;
use std::collections::HashMap;

fn sock_merchant(socks: &[i32]) -> i32 {
    let mut color_counts = HashMap::new();

    for &sock in socks {
        *color_counts.entry(sock).or_insert(0) += 1;
    }

    color_counts.values().map(|&count| count / 2).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let socks: Vec<i32> = input.split_whitespace()
                               .map(|s| s.parse().unwrap())
                               .collect();

    println!("{}", sock_merchant(&socks));
}
