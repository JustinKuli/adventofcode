#![allow(unused_variables)]

use std::error::Error;
use std::collections::HashMap;

fn main() {
    let data = read_data(String::from("formatted.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(data.clone()).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
    // 20352 is too low
}

fn part_one(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let pps = read_to_pp(data.clone())?;
    
    let mut geysers = HashMap::new(); 
    for pp in pps {
        let points = p1_points(pp);
        for p in points {
            let counter = geysers.entry(p).or_insert(0);
            *counter += 1;
        }
    }

    let mut cum_sum = 0;
    for (g, sum) in geysers {
        if sum > 1 {
            cum_sum += 1;
        }
    }

    return Ok(cum_sum.to_string());
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let pps = read_to_pp(data.clone())?;
    
    let mut geysers = HashMap::new(); 
    for pp in pps {
        let points = p2_points(pp);
        for p in points {
            let counter = geysers.entry(p).or_insert(0);
            *counter += 1;
        }
    }

    let mut cum_sum = 0;
    for (g, sum) in geysers {
        if sum > 1 {
            cum_sum += 1;
        }
    }

    return Ok(cum_sum.to_string());
}

struct PointPair {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn p1_points(pp: PointPair) -> Vec<String> {
    let mut v = Vec::new();

    if pp.x1 == pp.x2 { // line along y
        let mut i: i32;
        let max: i32;
        if pp.y1 > pp.y2 {
            i = pp.y2;
            max = pp.y1;
        } else {
            i = pp.y1;
            max = pp.y2;
        }

        while i <= max {
            v.push(format!("{},{}", pp.x1, i));
            i+=1;
        }
    } else if pp.y1 == pp.y2 { // line along x
        let mut i: i32;
        let max: i32;
        if pp.x1 > pp.x2 {
            i = pp.x2;
            max = pp.x1;
        } else {
            i = pp.x1;
            max = pp.x2;
        }

        while i <= max {
            v.push(format!("{},{}", i, pp.y1));
            i+=1;
        }
    }

    return v
}

fn p2_points(pp: PointPair) -> Vec<String> {
    let mut v = Vec::new();

    if pp.x1 == pp.x2 { // line along y
        let mut i: i32;
        let max: i32;
        if pp.y1 > pp.y2 {
            i = pp.y2;
            max = pp.y1;
        } else {
            i = pp.y1;
            max = pp.y2;
        }

        while i <= max {
            v.push(format!("{},{}", pp.x1, i));
            i+=1;
        }
    } else if pp.y1 == pp.y2 { // line along x
        let mut i: i32;
        let max: i32;
        if pp.x1 > pp.x2 {
            i = pp.x2;
            max = pp.x1;
        } else {
            i = pp.x1;
            max = pp.x2;
        }

        while i <= max {
            v.push(format!("{},{}", i, pp.y1));
            i+=1;
        }
    } else { // diagonal line (always exactly 45 degrees)
        let mut x = pp.x1;
        let mut y = pp.y1;

        let x_delta: i32;
        let too_far: i32;
        if x < pp.x2 {
            x_delta = 1;
            too_far = pp.x2 + 1;
        } else {
            x_delta = -1;
            too_far = pp.x2 -1;
        }

        let y_delta: i32;
        if y < pp.y2 {
            y_delta = 1;
        } else {
            y_delta = -1;
        }

        while x != too_far {
            v.push(format!("{},{}", x, y));
            x += x_delta;
            y += y_delta;
        }
    }

    return v
}


fn read_to_pp(data: Vec<csv::StringRecord>) -> Result<Vec<PointPair>, Box<dyn Error>> {
    let mut v = Vec::new();

    for line in data {
        let mut iter = line.into_iter();
        let pp = PointPair {
            x1: i32::from_str_radix(iter.next().unwrap_or("#"), 10)?,
            y1: i32::from_str_radix(iter.next().unwrap_or("#"), 10)?,
            x2: i32::from_str_radix(iter.next().unwrap_or("#"), 10)?,
            y2: i32::from_str_radix(iter.next().unwrap_or("#"), 10)?,
        };
        v.push(pp);
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
