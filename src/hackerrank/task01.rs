fn simple_array_sum(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 10, 11];
    println!("{}", simple_array_sum(&numbers)); // Виведе 31
}
