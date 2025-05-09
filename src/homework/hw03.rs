const W: usize = 20; 
const H: usize = 10; 

fn main() {
    let mut envelope = String::new();

    for y in 0..H {
        for x in 0..W {
            if y == 0 || y == H - 1 || x == 0 || x == W - 1 || x == y || x == W - y - 1 {
                envelope.push('*'); // Малюємо рамку та діагоналі
            } else {
                envelope.push(' '); // Порожній простір всередині
            }
        }
        envelope.push('\n'); // Перехід на новий рядок
    }

    print!("{}", envelope); // Виводимо конверт
}
