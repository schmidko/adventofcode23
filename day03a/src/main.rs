
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let filename: &str = "data.txt";
    let lines: Vec<Vec<char>> = read_file(filename).unwrap();
    let mut numbers: Vec<String> = Vec::new();
    let mut number = String::new();
    let mut flag: bool = false;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.is_numeric() {
                number.push(*c);
                let has_sign = check_special_sign(x, y, lines.clone());
                if has_sign {
                    flag = true;
                }
                if x+1 >= line.len() || !lines[y][x+1].is_numeric() {
                    if flag == true {
                        numbers.push(number.clone());
                    }
                    number = String::new();
                    flag = false;
                }
            }
        }
    }

    println!("{:?}", numbers);

    let mut sum = 0;
    for n in numbers {
        sum = sum + n.parse::<i32>().unwrap();
    }

    println!("{}", sum)
}

fn check_special_sign(x: usize, y: usize, matrix: Vec<Vec<char>>) -> bool {
    let mut has_sign = false;
   
    let pos: [[isize; 2]; 8] = [[-1, 1], [0, 1], [1, 1], [-1, -1], [0, -1], [1, -1], [-1, 0], [1, 0]];
    for coords in pos {
        let xpos = x.saturating_add_signed(coords[0]);
        let ypos = y.saturating_add_signed(coords[1]);
        
        if ypos >= matrix.len() || xpos >= matrix[0].len() {
            continue;
        }
        if !matrix[ypos][xpos].is_numeric() && matrix[ypos][xpos] != '.' {
            has_sign = true;
        }
    }

    has_sign
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
