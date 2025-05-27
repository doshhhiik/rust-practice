use std::io;

fn birthday_chocolate(s: &[i32], d: i32, m: usize) -> i32 {
    let mut count = 0;

    for i in 0..=s.len().saturating_sub(m) {
        if s[i..i + m].iter().sum::<i32>() == d {
            count += 1;
        }
    }

    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let s: Vec<i32> = input.split_whitespace()
                           .map(|s| s.parse().unwrap())
                           .collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<i32> = input.split_whitespace()
                                .map(|s| s.parse().unwrap())
                                .collect();

    let d = values[0];
    let m = values[1] as usize;

    println!("{}", birthday_chocolate(&s, d, m));
}
