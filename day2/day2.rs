use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/*
A X: Rock 1
B Y: Paper 2
C Z: Scissors 3
L 1, D 3, W 6

t2:
X = loose
Y = draw
Z = win
*/

fn main() -> io::Result<()> {
    let file = File::open("day2/input.txt")?;
    let reader = BufReader::new(file);
    let mut score: i32;
    let mut count: i32 = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        // Task 1
        // match line.as_str() {
        //     "A X" => score = 1 + 3,
        //     "A Y" => score = 2 + 6,
        //     "A Z" => score = 3 + 0,
        //     "B X" => score = 1 + 0,
        //     "B Y" => score = 2 + 3,
        //     "B Z" => score = 3 + 6,
        //     "C X" => score = 1 + 6,
        //     "C Y" => score = 2 + 0,
        //     "C Z" => score = 3 + 3,
        //     _ => score = 0,
        // };
        // Task 2
        match line.as_str() {
            "A X" => score = 3 + 0,
            "A Y" => score = 1 + 3,
            "A Z" => score = 2 + 6,
            "B X" => score = 1 + 0,
            "B Y" => score = 2 + 3,
            "B Z" => score = 3 + 6,
            "C X" => score = 2 + 0,
            "C Y" => score = 3 + 3,
            "C Z" => score = 1 + 6,
            _ => score = 0,
        };
        count += score;
    }
    println!("{}", count);

    Ok(())
}
