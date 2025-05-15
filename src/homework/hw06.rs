fn draw_tree(num_triangles: usize) {
    (1..=num_triangles).for_each(|triangle| {
        (0..triangle * 2).for_each(|row| {
            let stars = "*".repeat(row * 2 + 1);
            let spaces = " ".repeat(num_triangles * 2 - row);
            println!("{spaces}{stars}");
        });
    });
}

fn main() {
    let num_triangles = 6; 
    draw_tree(num_triangles);
}
