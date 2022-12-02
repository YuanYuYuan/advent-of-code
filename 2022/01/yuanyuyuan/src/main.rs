use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let mut data: Vec<u32> = read_to_string("./puzzle-input.txt")?
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|l| l.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    data.sort_by(|a, b| b.cmp(a));
    println!("[Part 1] Max: {}", data[0]);
    println!("[Part 2] Top three: {}", data[0] + data[1] + data[2]);
    Ok(())
}
