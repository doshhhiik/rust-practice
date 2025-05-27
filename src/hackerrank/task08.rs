use std::io;

fn time_conversion(s: &str) -> String {
    let (time, period) = s.split_at(8); // Відокремлюємо "AM"/"PM"
    let mut parts: Vec<&str> = time.split(':').collect();
    let hour: i32 = parts[0].parse().unwrap();

    parts[0] = match period {
        "AM" if hour == 12 => "00",
        "PM" if hour != 12 => &(hour + 12).to_string(),
        _ => parts[0],
    };

    parts.join(":")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    println!("{}", time_conversion(input));
}
