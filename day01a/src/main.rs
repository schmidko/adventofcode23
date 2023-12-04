
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
   
    let filename: &str = "data.txt";

    let _ = read_file_line_by_line(filename);
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;
    for line in reader.lines() {
        let mut first = String::from("");
        let mut last = String::from("");
       
        for c in line?.chars() {
            if c.is_numeric() {
                if first.is_empty() {
                    first = c.to_string();
                }
                last = c.to_string();
            }
        }
        let both = String::from(first + &last).parse::<i32>().unwrap();
        sum = sum + both;
    }
    println!("{}", sum);

    Ok(())
}