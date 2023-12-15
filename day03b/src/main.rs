
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn main() {
    let filename: &str = "data.txt";
    let lines: Vec<Vec<char>> = read_file(filename).unwrap();
    let mut stars: HashMap<String, String> = HashMap::new();
    let mut number = String::new();
    let mut flag: (bool, String) = (false, String::new());
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.is_numeric() {
                number.push(*c);
                let (has_sign, index) = check_special_sign(x, y, lines.clone());
                if has_sign {
                    flag.0 = true;
                    flag.1 = index;
                }
                if x+1 >= line.len() || !lines[y][x+1].is_numeric() {
                    
                    if flag.0 == true {
                        if stars.contains_key(&flag.1) {
                            stars.get_mut(&flag.1).unwrap().push_str("-");
                            stars.get_mut(&flag.1).unwrap().push_str(&number.clone());
                        } else {
                            stars.insert(flag.1, number.clone());
                        }
                    }
                    number = String::new();
                    flag = (false, String::new());
                }
            }
        }
    }

    let mut sum:i32 = 0;
    for ele in stars {
        let values:String = ele.1;
        if values.contains('-') {
            let parts: Vec<String> = values.split("-").map(String::from).collect();
            let n1:i32 = parts[0].clone().parse().unwrap();
            let n2:i32 = parts[1].clone().parse().unwrap();
            sum += n1*n2;
        }
    }

    println!("{}", sum)
}

fn check_special_sign(x: usize, y: usize, matrix: Vec<Vec<char>>) -> (bool, String) {
    let mut has_sign = false;
    let mut index: String = String::new();
   
    let pos: [[isize; 2]; 8] = [[-1, 1], [0, 1], [1, 1], [-1, -1], [0, -1], [1, -1], [-1, 0], [1, 0]];
    for coords in pos {
        let xpos = x.saturating_add_signed(coords[0]);
        let ypos = y.saturating_add_signed(coords[1]);
        
        if ypos >= matrix.len() || xpos >= matrix[0].len() {
            continue;
        }
        if matrix[ypos][xpos] == '*' {
            has_sign = true;
            index = xpos.to_string();
            index.push_str("-");
            index.push_str(&ypos.to_string());
        }
    }
    
    return (has_sign, index);
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
