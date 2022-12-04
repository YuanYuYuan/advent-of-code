use std::collections::HashSet;

fn main() {
    let priority = |c: char| -> i32 {
        if c.is_lowercase() {
            c as i32 - 'a' as i32 + 1
        } else {
            c as i32 - 'A' as i32 + 27
        }
    };
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/puzzle-input.txt"
    ));
    let total = input.clone()
        .lines()
        .fold(0, |sum, l| {
            let (first, second) = l.split_at(l.len() / 2);
            let set: HashSet<char> = first.chars().collect();
            let shared = second.chars().find(|c| set.contains(c)).unwrap();
            sum + priority(shared)
        });
    println!("Part 1: {}", total);

    let total = input
        .lines()
        .fold((0, 0, HashSet::new()), |(mut sum, idx, mut set), l| {
            if idx == 0 {
                set = l.chars().collect();
            } else if idx == 1 {
                set = l.chars().filter(|c| set.contains(c)).collect();
            } else {
                let shared = l.chars().find(|c| set.contains(c)).unwrap();
                sum += priority(shared);
            }
            (sum, (idx + 1) % 3, set)
        }).0;
    println!("Part 2: {}", total);
}
