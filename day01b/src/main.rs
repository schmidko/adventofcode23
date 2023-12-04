
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
        let mut line_new = String::from("");
        
        for c in line?.chars() {
            line_new = line_new + &c.to_string();
            line_new = line_new.replace("one", "1e");
            line_new = line_new.replace("two", "2o");
            line_new = line_new.replace("three", "3e");
            line_new = line_new.replace("four", "4r");
            line_new = line_new.replace("five", "5e");
            line_new = line_new.replace("six", "6x");
            line_new = line_new.replace("seven", "7n");
            line_new = line_new.replace("eight", "8t");
            line_new = line_new.replace("nine", "9e");
            
        }

        println!("{}", line_new);
        for c in line_new.chars() {
            if c.is_numeric() {
                if first.is_empty() {
                    first = c.to_string();
                }
                last = c.to_string();
            }
        }
        let both = String::from(first + &last).parse::<i32>().unwrap();
        sum = sum + both;
        println!("{}", both);
    }
    println!("{}", sum);

    Ok(())
}