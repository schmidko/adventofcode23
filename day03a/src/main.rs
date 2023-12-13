
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let filename: &str = "test.txt";
    let lines: Vec<Vec<char>> = read_file(filename).unwrap();
    let mut numbers: Vec<String> = Vec::new();
    let mut number = String::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.is_numeric() {
                number.push(*c);

                if !lines[y][x+1].is_numeric() {
                    numbers.push(number.clone());
                    number = String::new();
                }
                
            }
            
        }
    }

    println!("{:?}", numbers);
}

fn check_special_sign(x: usize, y:usize, matrix: Vec<Vec<char>>) -> bool {
    const RADIX: u32 = 10;
    if matrix[y][x].to_digit(RADIX) > Some(3) {
        true
    } else {
        false
    }
}

fn read_file(filepath: &str) -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let chars: Vec<char> = line?.chars().collect::<Vec<_>>();
        data.push(chars);
    }

    Ok(data)
}
