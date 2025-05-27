use std::io;

fn breaking_records(scores: &[i32]) -> (i32, i32) {
    let mut highest = scores[0];
    let mut lowest = scores[0];
    let mut high_count = 0;
    let mut low_count = 0;

    for &score in scores.iter().skip(1) {
        if score > highest {
            highest = score;
            high_count += 1;
        } else if score < lowest {
            lowest = score;
            low_count += 1;
        }
    }

    (high_count, low_count)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let scores: Vec<i32> = input.split_whitespace()
                                .map(|s| s.parse().unwrap())
                                .collect();

    let (high, low) = breaking_records(&scores);
    println!("{} {}", high, low);
}
