#![allow(unused_variables)]

use std::error::Error;

fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(data.clone(), 80).expect("Failed to get answer 1");
    println!("Part one fish heads: {}", ans1);

    // part two just needed to update the code to u64 instead of the usual i32
    let ans2 = part_one(data.clone(), 256).expect("Failed to get answer 2");
    println!("Part two fish heads: {}", ans2);
}

fn part_one(data: Vec<csv::StringRecord>, days: i32) -> Result<String, Box<dyn Error>> {
    let mut ss = sea_state(data)?;

    for i in 0..days {
        ss = increment(ss);
    }
    
    let mut acc: u64 = 0;
    acc += ss.s0;
    acc += ss.s1;
    acc += ss.s2;
    acc += ss.s3;
    acc += ss.s4;
    acc += ss.s5;
    acc += ss.s6;
    acc += ss.s7;
    acc += ss.s8;

    return Ok(acc.to_string());
}

fn increment(ss: SeaState) -> SeaState {
    return SeaState{
        s0: ss.s1,
        s1: ss.s2,
        s2: ss.s3,
        s3: ss.s4,
        s4: ss.s5,
        s5: ss.s6,
        s6: ss.s7 + ss.s0,
        s7: ss.s8,
        s8: ss.s0,
    };
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    return Ok(String::from(""));
}

struct SeaState {
    s0: u64,
    s1: u64,
    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
}

#[allow(dead_code)]
fn sea_state(data: Vec<csv::StringRecord>) -> Result<SeaState, Box<dyn Error>> {
    let mut ss = SeaState{
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
        s4: 0,
        s5: 0,
        s6: 0,
        s7: 0,
        s8: 0,
    };

    for line in data {
        for item in line.into_iter() {
            let t = i32::from_str_radix(item, 10)?;
            match t {
                0 => ss.s0 += 1,
                1 => ss.s1 += 1,
                2 => ss.s2 += 1,
                3 => ss.s3 += 1,
                4 => ss.s4 += 1,
                5 => ss.s5 += 1,
                6 => ss.s6 += 1,
                7 => ss.s7 += 1,
                8 => ss.s8 += 1,
                _ => return Err("bad input".into()),
            }
        }
    }

    return Ok(ss);
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
