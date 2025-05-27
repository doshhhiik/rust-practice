use std::io;

fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade < 38 {
            grade
        } else {
            let next_multiple_of_five = ((grade / 5) + 1) * 5;
            if next_multiple_of_five - grade < 3 {
                next_multiple_of_five
            } else {
                grade
            }
        }
    }).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    let mut grades = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    grades = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for grade in grading_students(&grades) {
        println!("{}", grade);
    }
}
