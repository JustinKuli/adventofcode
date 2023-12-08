#![allow(unused_variables)]

use std::error::Error;

fn main() {
    let data = read_data(String::from("small-data.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(data.clone()).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

fn part_one(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let mut acc = 0;

    // this sample code counts lines in the file which match the length of the first word.
    // Eg, if the first line is `foo`, it will count lines that are `3`.
    let length = first_word_len(data.clone())?;
    let search_for = length.to_string();

    let strings = ss_string(data.clone())?;
    for s in strings.into_iter() {
        if s == search_for {
            acc += 1;
        }
    }
    
    return Ok(acc.to_string());
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    return Ok(String::from(""));
}

#[allow(dead_code)]
fn ss_string(data: Vec<csv::StringRecord>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut v = Vec::new();

    for line in data {
        for item in line.into_iter() {
            v.push(item.to_string());
        }
    }

    return Ok(v);
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

// Returns the length of the first word of the first line in the data.
// Uses iterators to prevent going through the entire data.
#[allow(dead_code)]
fn first_word_len(data: Vec<csv::StringRecord>) -> Result<usize, &'static str> {
    let line1 = data.iter().next();
    if line1.is_none() {
        return Err("Could not get first line")
    }

    let word1 = line1.unwrap().into_iter().next();
    if word1.is_none() {
        return Err("Could not get first word");
    }

    return Ok(word1.unwrap().len())
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
