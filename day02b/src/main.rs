use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let filename: &str = "data.txt";
    let lines = read_file(filename).unwrap();
    let re1 = Regex::new(r"Game.*:").unwrap();
    let re2 = Regex::new(r"[A-Za-z ]").unwrap();
    let mut result = 0;
    for line in lines {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let line = re1.replace_all(line.as_str(), "");
        let parts = line.split(";");
        for part in parts {
            let parti = part.split(",");
            for single in parti {
                if single.contains("red") {
                    let num = re2.replace_all(single, "");
                    let my_int: i32 = num.parse().unwrap();
                    if red < my_int {
                        red = my_int;
                    }
                }
                if single.contains("green") {
                    let num = re2.replace_all(single, "");
                    let my_int: i32 = num.parse().unwrap();
                    if green < my_int {
                        green = my_int;
                    }
                }
                if single.contains("blue") {
                    let num = re2.replace_all(single, "");
                    let my_int: i32 = num.parse().unwrap();
                    if blue < my_int {
                        blue = my_int;
                    }
                }
            }
        }
        //println!("{} {} {}", red, green, blue);
        result += red * green * blue;
    }
    println!("{}", result);
}

fn read_file(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut data: Vec<String> = Vec::new();
    for line in reader.lines() {
        data.push(line?);
    }

    Ok(data)
}
