#![allow(unused_variables)]

use std::error::Error;

fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(data.clone()).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

fn part_one(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let positions = ss_i32(data, 10)?;
    
    let mut min = 0;
    let mut max = 0;
    let mut max_cost = 0;
    for p in positions.clone() {
        max_cost += p;
        if p < min {
            min = p;
        }
        if p > max {
            max = p;
        }
    }

    let mut min_cost = max_cost;
    for p in min..max {
        let cost = fuel_cost(positions.clone(), p);
        if cost < min_cost {
            min_cost = cost;
        }
    }
    
    return Ok(min_cost.to_string());
}

fn fuel_cost(data: Vec<i32>, pos: i32) -> i32 {
    let mut acc = 0;
    for d in data.clone() {
        if d > pos {
            acc += d-pos;
        } else {
            acc += pos-d;
        }
    }
    return acc;
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let positions = ss_i32(data, 10)?;
    
    let mut min = 0;
    let mut max = 0;
    for p in positions.clone() {
        if p < min {
            min = p;
        }
        if p > max {
            max = p;
        }
    }

    let mut min_cost = i32::MAX;
    for p in min..max {
        let cost = adv_fuel_cost(positions.clone(), p);
        if cost < min_cost {
            min_cost = cost;
        }
    }
    
    return Ok(min_cost.to_string());
}

fn adv_fuel_cost(data: Vec<i32>, pos: i32) -> i32 {
    let mut acc = 0;
    for d in data.clone() {
        let n: i32;
        if d > pos {
            n = d-pos;
        } else {
            n = pos-d;
        }
        acc += n*(n+1)/2
    }
    return acc;
}

#[allow(dead_code)]
fn ss_i32(data: Vec<csv::StringRecord>, radix: u32) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut v = Vec::new();

    for line in data {
        for item in line.into_iter() {
            let n = i32::from_str_radix(item, radix)?;
            v.push(n);
        }
    }

    return Ok(v);
}

fn read_data(path: String) -> Result<Vec<csv::StringRecord>, Box<dyn Error>> {
    let mut v = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        v.push(record);
    }

    return Ok(v);
}
