use std::io;

fn birthday_cake_candles(candles: &[i32]) -> usize {
    let max_height = candles.iter().max().unwrap();
    candles.iter().filter(|&&x| x == *max_height).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let candles: Vec<i32> = input.split_whitespace()
                                .map(|s| s.parse().unwrap())
                                .collect();

    println!("{}", birthday_cake_candles(&candles));
}
