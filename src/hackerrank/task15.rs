use std::io;

fn divisible_sum_pairs(n: usize, k: i32, arr: &[i32]) -> i32 {
    let mut count = 0;

    for i in 0..n {
        for j in i + 1..n {
            if (arr[i] + arr[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (n, k) = (values[0] as usize, values[1]);

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    println!("{}", divisible_sum_pairs(n, k, &arr));
}
