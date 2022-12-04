use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("day1/input.txt")?;
    let reader = BufReader::new(file);
    let mut count = 0;
    let mut max = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.parse::<String>().unwrap() != "" {
            count += line.parse::<i32>().unwrap_or(0);
            // println!("{}", count);
        } else {
            if count > max {
                max = count;
            }
            count = 0;
       }
    }
    println!("{}", max);

    Ok(())
}
