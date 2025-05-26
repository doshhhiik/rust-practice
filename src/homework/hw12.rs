use std::collections::BTreeMap;

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point, // Ліва верхня точка
    b: Point, // Права нижня точка
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut events = vec![];

    for rect in xs.iter() {
        events.push((rect.a.x, rect.a.y, rect.b.y, 1)); // Ліва межа
        events.push((rect.b.x, rect.a.y, rect.b.y, -1)); // Права межа
    }

    events.sort(); \

    let mut active_intervals = BTreeMap::new();
    let mut prev_x = 0;
    let mut total_area = 0;

    for (x, y1, y2, typ) in events {
        let mut height = 0;
        let mut prev_y = 0;

        for (&y_start, &count) in active_intervals.iter() {
            if count > 0 {
                if prev_y < y_start {
                    height += y_start - prev_y;
                }
                prev_y = y_start;
            }
        }

        total_area += height * (x - prev_x);
        prev_x = x;

        *active_intervals.entry(y1).or_insert(0) += typ;
        *active_intervals.entry(y2).or_insert(0) -= typ;
    }

    total_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    println!("Фактична зайнята площа: {}", area_occupied(&test_data()));
}
