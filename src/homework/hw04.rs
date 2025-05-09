const SIZE: usize = 9; 

fn main() {
    let mut diamond = String::new();

    for i in 0..SIZE {
        let spaces = (SIZE / 2).abs_diff(i);
        let stars = SIZE - 2 * spaces;

        diamond.push_str(&" ".repeat(spaces));
        diamond.push_str(&"*".repeat(stars));
        diamond.push('\n');
    }

    print!("{}", diamond); 
}
