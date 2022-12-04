use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("day1/input.txt")?;
    let reader = BufReader::new(file);
    let mut count = 0;
    let mut vec: Vec<i32> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.parse::<String>().unwrap() != "" {
            count += line.parse::<i32>().unwrap_or(0);
        } else {
            vec.push(count);
            count = 0;
       }
    }
    vec.sort();
    vec.reverse();
    println!("{}", vec[0] + vec[1] + vec[2]);

    Ok(())
}
