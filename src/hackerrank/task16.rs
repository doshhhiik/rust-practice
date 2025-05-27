use std::io;
use std::collections::HashMap;

fn migratory_birds(birds: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for &bird in birds {
        *counts.entry(bird).or_insert(0) += 1;
    }

    counts.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&bird, _)| bird)
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let birds: Vec<i32> = input.split_whitespace()
                               .map(|s| s.parse().unwrap())
                               .collect();

    println!("{}", migratory_birds(&birds));
}
