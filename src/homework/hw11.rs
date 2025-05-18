use rand::Rng;

// функція для генерації випадкового вектора
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// функція для знаходження мінімальної суми сусідніх чисел
fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = (i, i + 1);
        }
    }

    (min_index.0, min_index.1, min_sum)
}

// функція для гарного виводу результату
fn print_vector_info(data: &[i32]) {
    println!("indexes: {:?}", (0..data.len()).collect::<Vec<usize>>());
    println!("data: {:?}", data);

    let (idx1, idx2, min_sum) = min_adjacent_sum(data);
    println!("indexes:");
    println!("        \\__ __/");
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[idx1], data[idx2], min_sum, idx1, idx2);
}

fn main() {
    let vec = gen_random_vector(20);
    print_vector_info(&vec);
}
