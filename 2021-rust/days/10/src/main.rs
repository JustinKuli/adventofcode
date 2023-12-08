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
    let mut acc = 0;

    let lines = ss_string(data)?;
    for l in lines {
        acc += p1_score(l);
    }
    
    return Ok(acc.to_string());
}

fn p1_score(line: String) -> i32 {
    let mut active_chunks = Vec::new();
    // use push and pop to make it like a stack

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                active_chunks.push(c);
            },
            ')' => {
                let last = active_chunks.pop().unwrap();
                if last != '(' {
                    return 3;
                }
            },
            ']' => {
                let last = active_chunks.pop().unwrap();
                if last != '[' {
                    return 57;
                }
            },
            '}' => {
                let last = active_chunks.pop().unwrap();
                if last != '{' {
                    return 1197;
                }
            },
            '>' => {
                let last = active_chunks.pop().unwrap();
                if last != '<' {
                    return 25137;
                }
            },
            _ => {
                println!("Unkown character: {}", c);
                return 0
            }
        }
    }

    return 0
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let mut scores = Vec::new();

    let lines = ss_string(data)?;
    for l in lines {
        let score = p2_score(l);
        if score != 0 {
            scores.push(score);
        }
    }
    scores.sort();

    let ans = scores.get(scores.len()/2).unwrap();
    
    return Ok(ans.to_string());
}

fn p2_score(line: String) -> u64 {
    let mut active_chunks = Vec::new();
    // use push and pop to make it like a stack

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                active_chunks.push(c);
            },
            ')' => {
                let last = active_chunks.pop().unwrap();
                if last != '(' {
                    return 0;
                }
            },
            ']' => {
                let last = active_chunks.pop().unwrap();
                if last != '[' {
                    return 0;
                }
            },
            '}' => {
                let last = active_chunks.pop().unwrap();
                if last != '{' {
                    return 0;
                }
            },
            '>' => {
                let last = active_chunks.pop().unwrap();
                if last != '<' {
                    return 0;
                }
            },
            _ => {
                println!("Unkown character: {}", c);
                return 0
            }
        }
    }
    
    let mut score: u64 = 0;
    while !active_chunks.is_empty() {
        let last = active_chunks.pop().unwrap();
        match last {
            '(' => {
                score *= 5;
                score += 1;
            },
            '[' => {
                score *= 5;
                score += 2;
            },
            '{' => {
                score *= 5;
                score += 3;
            },
            '<' => {
                score *= 5;
                score += 4;
            },
            _ => {
                println!("Unkown character: {}", last);
                return 0
            }

        }
    }

    return score;
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
