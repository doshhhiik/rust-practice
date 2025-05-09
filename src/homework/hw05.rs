fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let a = 24;
    let b = 60;
    println!("GCD of {} and {} is {}", a, b, gcd(a, b));
}

#[test]
fn test_gcd() {
    let test_cases = [
        ((24, 60), 12),
        ((15, 9), 3),
        ((140, 40), 20),
        ((100, 10), 10),
        ((37, 11), 1),
        ((120, 90), 30),
    ];

    for ((a, b), expected) in test_cases.iter() {
        assert_eq!(gcd(*a, *b), *expected);
    }
}
