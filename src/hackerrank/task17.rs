use std::io;

fn bon_appetit(bill: &[i32], k: usize, b_charged: i32) {
    let total_cost: i32 = bill.iter().sum();
    let fair_share = (total_cost - bill[k]) / 2;

    if b_charged == fair_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b_charged - fair_share);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (n, k) = (values[0], values[1]);

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let bill: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b_charged: i32 = input.trim().parse().unwrap();

    bon_appetit(&bill, k, b_charged);
}
