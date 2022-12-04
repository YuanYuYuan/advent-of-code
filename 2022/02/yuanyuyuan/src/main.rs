fn main() {
    let round_score = |a, b| -> i32 {
        // 0: draw, 1: win, 2: lose
        let result = (b - a + 3) % 3;
        // lose: 0, draw: 3, win: 6
        ((result + 1) % 3) * 3
    };
    let score = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/puzzle-input.txt"))
        .lines()
        .into_iter()
        .map(|l| l.chars().filter(|&c| c.is_ascii_uppercase()).collect())
        .fold((0, 0), |(total1, total2), chars: Vec<_>| {
            // part 1
            let a = chars[0] as i32 - 'A' as i32;
            let b = chars[1] as i32 - 'X' as i32;
            // part 2
            let c = (a + b - 1 + 3) % 3;
            (
                total1 + b + 1 + round_score(a, b),
                total2 + c + 1 + round_score(a, c),
            )
        });
    println!("Part 1: {}", score.0);
    println!("Part 2: {}", score.1);
}
