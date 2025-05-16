fn compare_triplets(a: &[i32], b: &[i32]) -> (i32, i32) {
    let (mut alice_score, mut bob_score) = (0, 0);

    a.iter().zip(b.iter()).for_each(|(a_val, b_val)| {
        match a_val.cmp(b_val) {
            std::cmp::Ordering::Greater => alice_score += 1,
            std::cmp::Ordering::Less => bob_score += 1,
            _ => {} // Нічого не додаємо при рівних значеннях
        }
    });

    (alice_score, bob_score)
}

fn main() {
    let alice = vec![5, 6, 7];
    let bob = vec![3, 6, 10];

    let (alice_score, bob_score) = compare_triplets(&alice, &bob);
    println!("{} {}", alice_score, bob_score);
}
