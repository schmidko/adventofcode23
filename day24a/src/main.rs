use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let filename: &str = "data.txt";
    let rows: Vec<Vec<f64>> = read_file(filename).unwrap();
    let length = rows.len();

    let mut count = 0;
    for i in 0..length-1 {
        for ii in i + 1..rows.len() {
            let result = calc_intersection(
                    rows[i].clone(),
                    rows[ii].clone(),
                );
            if result == true {
                count += 1;
            }
            println!("-->{}", result);
        }
    }
    println!("result:{}", count);
}

fn calc_intersection(rowa: Vec<f64>, rowb: Vec<f64>) -> bool {
    let min: f64 = 200000000000000.0;
    let max: f64 = 400000000000000.0;

    let x1: f64 = rowa[0];
    let y1: f64 = rowa[1];
    let x2: f64 = rowa[0] + rowa[3];
    let y2: f64 = rowa[1] + rowa[4];

    let x3: f64 = rowb[0];
    let y3: f64 = rowb[1];
    let x4: f64 = rowb[0] + rowb[3];
    let y4: f64 = rowb[1] + rowb[4];

    if (x1 == x2 && y1 == y2) || (x3 == x4 && y3 == y4) {
        return false;
    }

    let denominator: f64 = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    if denominator == 0.0 {
        return false;
    }

    let ua: f64 = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;
    let ub: f64 = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denominator;

    let x: f64 = x1 + ua * (x2 - x1);
    let y: f64 = y1 + ua * (y2 - y1);

    //println!("{}-{}", x, y);

    // check left for past crossings
    if rowa[3].signum() == -1.0 && x1 < x
        || rowa[3].signum() == 1.0 && x1 > x
        || rowa[4].signum() == -1.0 && y1 < y
        || rowa[4].signum() == 1.0 && y1 > y {
        return false;
    }

    // check right for past crossings
    if rowb[3].signum() == -1.0 && x3 < x
        || rowb[3].signum() == 1.0 && x3 > x
        || rowb[4].signum() == -1.0 && y3 < y
        || rowb[4].signum() == 1.0 && y3 > y {
        return false;
    }

    if x <= max && x >= min && y <= max && y >= min {
        return true;
    } else {
        return false;
    }
}

fn read_file(filepath: &str) -> Result<Vec<Vec<f64>>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<f64>> = Vec::new();
    for line in reader.lines() {
        let li = line.unwrap();
        let li = li.replace("@", ",");
        let parts: Vec<&str> = li.split(",").collect();
        let mut row: Vec<f64> = Vec::new();

        row.push(parts[0].trim().parse().unwrap());
        row.push(parts[1].trim().parse().unwrap());
        row.push(parts[2].trim().parse().unwrap());
        row.push(parts[3].trim().parse().unwrap());
        row.push(parts[4].trim().parse().unwrap());
        row.push(parts[5].trim().parse().unwrap());
        data.push(row);
    }

    Ok(data)
}
