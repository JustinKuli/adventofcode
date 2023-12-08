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
}

fn part_one(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let mut acc = 0;

    for line in data {
        let mut after_bar = false;
        for item in line.into_iter() {
            if after_bar {
                match item.len() {
                    2 | 4 | 3 | 7 => acc += 1,
                    _ => {},
                }
            } else {
                if item == "|" {
                    after_bar = true;
                }
            }
        } 
    }
    
    return Ok(acc.to_string());
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let mut cum_sum = 0;

    for line in data {
        let map = find_chars(line.clone());
        let mut sum = 0;
        let mut factor = 1000;
        let mut after_bar = false;
        for item in line.into_iter() {
            if after_bar {
                sum += factor * score(item.to_string(), map.clone());
                factor /= 10;
            } else {
                if item == "|" {
                    after_bar = true
                }
            }
        }
        cum_sum += sum;
    }
    
    return Ok(cum_sum.to_string());
}

fn find_chars(line: csv::StringRecord) -> HashMap<String, String> {
    let chars: [&str; 7] = ["a", "b", "c", "d", "e", "f", "g"];

    let mut found_chars = HashMap::new();

    let mut bef_pattern = "".to_string();
    let mut cd_pattern = "".to_string();
    for item in line.into_iter() {
        if item == "|" {
            break
        }
        bef_pattern += &item.to_string();
        let l = item.len();
        if l != 2 && l != 4 && l != 3 && l != 7 {
            cd_pattern += &item.to_string();
        }
    }

    for c in chars {
        match bef_pattern.matches(c).count() {
            6 => {
                found_chars.insert(c.to_string(), "b".to_string());
                continue;
            },
            4 => {
                found_chars.insert(c.to_string(), "e".to_string());
                continue;
            },
            9 => {
                found_chars.insert(c.to_string(), "f".to_string());
                continue;
            },
            _ => {},
        }
        match cd_pattern.matches(c).count() {
            4 => {
                found_chars.insert(c.to_string(), "c".to_string());
                continue;
            },
            5 => {
                found_chars.insert(c.to_string(), "d".to_string());
                continue;
            },
            _ => {},
        }
    }

    for c in chars {
        if found_chars.contains_key(&c.to_string()) {
            continue
        }
        match bef_pattern.matches(c).count() {
            8 => found_chars.insert(c.to_string(), "a".to_string()),
            7 => found_chars.insert(c.to_string(), "g".to_string()),
            _ => println!("Unexpected case"),
        }
    }

    return found_chars;
}

fn score(inp: String, map: HashMap<String, String>) -> i32 {
    let mut disp = "".to_string();
    for c in inp.chars() {
        match map.get(&c.to_string()) {
            Some(thing) => disp += thing,
            None => println!("Uh-oh")
        }
    }

    match disp.len() {
        2 => return 1,
        4 => return 4,
        3 => return 7,
        7 => return 8,
        6 => {
            // 0 or 6 or 9
            if disp.contains("c") && disp.contains("e") {
                return 0;
            } else if disp.contains("d") && disp.contains("e") {
                return 6;
            } else {
                return 9;
            }
        },
        5 => {
            // 2 or 3 or 5
            if disp.contains("b") {
                return 5;
            } else if disp.contains("e") {
                return 2;
            } else {
                return 3;
            }
        },
        _ => {
            println!("this is not good");
            return 0;
        },
    }
}

#[allow(dead_code)]
fn to_strings(data: Vec<csv::StringRecord>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut v = Vec::new();

    for line in data {
        for item in line.into_iter() {
            v.push(item.to_string());
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
