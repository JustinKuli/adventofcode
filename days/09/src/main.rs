#![allow(unused_variables)]

use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;

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
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
struct BasinPart {
    start: i32,
    end: i32,
    size: i32,
    adj: Vec<i32>,
    id: i32,
}

fn get_basin_parts(line: String, start_id: i32) -> Vec<BasinPart> {
    let mut v = Vec::new();

    let mut in_basin = false;
    let mut i = 0;
    let mut bp = BasinPart{
        start: 0,
        end: 0,
        size: 0,
        adj: Vec::new(),
        id: start_id,
    };
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
                bp = BasinPart{
                    start: i,
                    end: i,
                    size: 0,
                    adj: Vec::new(),
                    id: start_id + i,
                };
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

    let mut map = HashMap::<i32, BasinPart>::new();
    let mut parts_above = Vec::<BasinPart>::new();

    let mut line_num = 1000;
    for line in lines {
        let parts = get_basin_parts(line, line_num);
        for curr in parts.clone() {
            map.insert(curr.id, curr.clone());
            for prev in parts_above.clone() {
                if overlap(curr.clone(), prev.clone()) {
                    let mut new_curr = map.remove(&curr.id).unwrap();
                    new_curr.adj.push(prev.id);
                    map.remove(&curr.id);
                    map.insert(curr.id, new_curr);

                    let mut new_prev = map.remove(&prev.id).unwrap();
                    new_prev.adj.push(curr.id);
                    map.insert(prev.id, new_prev);
                }
            }
        }
        parts_above = parts;
        line_num += 1000
    }

    let mut sizes = HashMap::<i32, i32>::new();
    let mut visited = HashSet::<i32>::new();

    for (idx, bp) in map.clone() {
        if visited.contains(&idx) {
            // was already present.
            continue
        }
        visited.insert(idx);

        let mut size = bp.size;
        let mut to_visit = HashSet::<i32>::new();
        for i in bp.adj {
            to_visit.insert(i);
        }

        while !to_visit.is_empty() {
            let v_i = to_visit.iter().cloned().next().unwrap();
            to_visit.remove(&v_i);
            visited.insert(v_i);

            let v = map.get(&v_i).unwrap();
            size += v.size;
            for adj in v.adj.clone() {
                if !visited.contains(&adj) {
                    to_visit.insert(adj);
                }
            }
        }

        sizes.insert(idx, size);
    }

    let mut all_sizes: Vec<i32> = sizes.values().cloned().collect();
    all_sizes.sort();

    let mut acc = 1;
    for i in 0..3 {
        let s = all_sizes.pop().unwrap();
        println!("{}",s);
        acc *= s;
    }

    return Ok(acc.to_string());
    // 856716 is too low?
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
