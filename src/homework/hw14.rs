use itertools::permutations;


fn find_solutions() -> Vec<(u8, u8, u8, u8, u8, u8, u8, u8)> {
    let digits = (1..=8).collect::<Vec<u8>>();
    let mut solutions = Vec::new();

    for perm in permutations(digits, 8) {
        let (m, u, x, a, s, l, o, n) = (perm[0], perm[1], perm[2], perm[3], perm[4], perm[5], perm[6], perm[7]);

        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        if muxa * a == slon {
            solutions.push((m, u, x, a, s, l, o, n));
        }
    }

    solutions
}

fn main() {
    let solutions = find_solutions();
    println!("Знайдено {} рішень:", solutions.len());

    for (m, u, x, a, s, l, o, n) in solutions {
        println!("  {}{}{}{} × {} = {}{}{}{}", m, u, x, a, a, s, l, o, n);
    }
}

#[test]
fn test_solutions() {
    let solutions = find_solutions();
    assert!(solutions.len() > 0); 
}
