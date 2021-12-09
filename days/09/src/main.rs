#![allow(unused_variables)]

use std::error::Error;
use std::collections::HashMap;

fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(data.clone()).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

fn part_one(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let lines = ss_string(data.clone())?;
    let linelen = first_word_len(data.clone())?;
    let j_max:i32 = linelen.try_into()?;
    let i_max:i32 = lines.len().try_into()?;
    
    let mut acc = 0;

    let mut i:i32 = 0;
    while i < i_max {
        let mut j:i32 = 0;
        while j < j_max  {
            acc += score_point(lines.clone(), i, j);
            j+=1;
        }
        i+=1;
    }

    return Ok(acc.to_string());
}

fn score_point(data: Vec<String>, i: i32, j: i32) -> i32 {
    let l: i32;
    match lookup(data.clone(), i, j) {
        Some(ans) => l = ans,
        None => return 0
    }

    let neighbors = get_neighbors(data.clone(), i, j);

    for n in neighbors {
        if l >= n {
            return 0
        }
    }

    return l+1;
}

fn get_neighbors(data: Vec<String>, i: i32, j: i32) -> Vec<i32> {
    let mut v = Vec::new();

    match lookup(data.clone(), i-1, j) {
        Some(n) => v.push(n),
        None => {}
    }

    match lookup(data.clone(), i+1, j) {
        Some(n) => v.push(n),
        None => {}
    }

    match lookup(data.clone(), i, j-1) {
        Some(n) => v.push(n),
        None => {}
    }

    match lookup(data.clone(), i, j+1) {
        Some(n) => v.push(n),
        None => {}
    }

    return v;
}

fn lookup(data: Vec<String>, i: i32, j: i32) -> Option<i32> {
    let ii: usize;
    let jj: usize;

    match usize::try_from(i) {
        Ok(x) => ii = x,
        Err(e) => return None
    }

    match usize::try_from(j) {
        Ok(x) => jj = x,
        Err(e) => return None
    }

    let line = data.get(ii)?;
    let c = line.chars().nth(jj)?;
    let n = i32::from_str_radix(&c.to_string(), 10);
    match n {
        Ok(num) => return Some(num),
        Err(e) => return None,
    }
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq, Eq, Hash)]
struct BasinPart {
    start: i32,
    end: i32,
    size: i32,
}

fn get_basin_parts(line: String) -> Vec<BasinPart> {
    let mut v = Vec::new();

    let mut in_basin = false;
    let mut i = 0;
    let mut bp = BasinPart{start: 0, end: 0, size: 0};
    for c in line.chars() {
        // println!("{}", c);
        if c == '9' {
            if in_basin {
                // end basin part
                bp.size = bp.end - bp.start + 1;
                v.push(bp.clone());
                in_basin = false;
            }
        } else {
            if in_basin {
                // continue basin part
                bp.end = i;
            } else {
                // start basin part
                bp = BasinPart{start: i, end: i, size: 0};
                in_basin = true;
            }
        } 
        i += 1;
    }
    if in_basin {
        // end basin part
        bp.size = bp.end - bp.start + 1;
        v.push(bp.clone());
    }

    return v;
}

fn overlap(one: BasinPart, two: BasinPart) -> bool {
    if one.start == two.start {
        return true
    } else if one.start < two.start {
        return one.end >= two.start
    } else {
        return two.end >= one.start
    }
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let lines = ss_string(data.clone())?;

    let mut basin_sizes = HashMap::new(); // index -> size
    let mut basin_map = HashMap::new(); // bps -> index
    let mut map_index = 0;

    let mut last_bps = Vec::new();
    for line in lines {
        let new_bps = get_basin_parts(line);
        for bp in new_bps.clone() {
            let mut overlaps = Vec::new();
            for prev in last_bps.clone() {
                if overlap(bp, prev) {
                    overlaps.push(prev)
                }
            }
            match overlaps.len() {
                0 => {
                    // new item in basin_sizes
                    basin_map.insert(bp, map_index);
                    basin_sizes.insert(map_index, bp.size);
                    map_index += 1;
                },
                1 => {
                    // update existing item in basin_sizes
                    let overlapping_bp = overlaps.get(0).unwrap();
                    let index = basin_map.get(overlapping_bp).unwrap();

                    // basically a += here:
                    let basin = basin_sizes.get(index).unwrap();
                    basin_sizes.insert(*index, basin + bp.size);
                },
                _ => {
                    // combine all the existing items, remove extras.
                }
            }
        }
        last_bps = new_bps;
    }

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

// #[allow(dead_code)]
// fn get_neighbors_diags(data: Vec<String>, i: usize, j: usize) -> Vec<i32> {
//     let mut v = Vec::new();
//     for i_d in 0..3 {
//         // Seriously rust? Or maybe I'm missing something :(
//         match i32::try_from(i + i_d) {
//             Ok(ii) => {
//                 if ii-1 < 0 {
//                     continue;
//                 }
//                 let iii = usize::try_from(ii-1).unwrap();
//                 for j_d in 0..3 {
//                     match i32::try_from(j + j_d) {
//                         Ok(jj) => {
//                             if jj-1 < 0 {
//                                 continue;
//                             }
//                             let jjj = usize::try_from(jj-1).unwrap();
//                             match lookup(data.clone(), iii, jjj) {
//                                 Some(n) => v.push(n),
//                                 None => {}
//                             }
//                         },
//                         Err(e) => {
//                             continue;
//                         }
//                     }
//                 }
//             },
//             Err(e) => {
//                 continue;
//             }
//         }
//     }
//     return v;
// }
