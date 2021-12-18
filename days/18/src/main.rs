#![allow(unused_variables)]

use std::error::Error;
use regex::Regex;

fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1); // takes about 25 seconds on my machine

    let ans2 = part_two(data.clone()).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2); // takes about 5.5 minutes on my machine
}

const DEBUG: bool = false;

fn part_one(data: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut sum = String::from("");
    let mut first_time = true;

    for item in data {
        if first_time {
            sum = item;
            first_time = false;
            continue;
        }

        if DEBUG { println!("adding: {}   and   {}", sum, item); }
        sum = format!("[{},{}]", sum, item);

        if DEBUG { println!("reducing:      {}", sum); }
        sum = reduce(sum.to_string());
    }

    return Ok(magnitude(sum));
}

fn part_two(data: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut largest = 0;

    let length = data.len();
    let mut i = 0;
    for sn1 in data.clone() {
        println!("Checking pair {}/{}", i, length);
        let mut j = 0;
        for sn2 in data.clone() {
            if i == j {
                j += 1;
                continue;
            }

            let sum = format!("[{},{}]", sn1, sn2);
            let mag_str = magnitude(reduce(sum));
            let mag = u64::from_str_radix(&mag_str, 10).unwrap();
            if mag > largest {
                largest = mag;
            }

            j += 1;
        }
        i += 1;
    }

    return Ok(largest.to_string());
}

fn reduce(sn_num: String) -> String {
    let mut result = sn_num.clone();

    loop {
        match try_explode(result.clone()) {
            Some(exploded) => {
                result = exploded;
                continue;
            },
            None => {
                // do nothing
            }
        }

        match try_split(result.clone()) {
            Some(split) => {
                result = split;
            },
            None => {
                return result;
            }
        }
    }
}

fn try_explode(sn_num: String) -> Option<String> {
    let pair_regex = Regex::new(r"^\[\d+,\d+\]").unwrap();

    let mut nest_level = 0;
    let mut num_len = 0;
    let mut num_val = 0;

    let mut nums = Vec::new();
    let mut fstring = String::from("");

    let mut i = 0;
    let mut exploding = false;
    let mut add_right = 0;

    let mut char_iter = sn_num.chars();
    loop {
        // Iterating more manually lets us advance inside the loop sometimes.
        let c: char;
        match char_iter.next() {
            Some(nc) => {
                c = nc;
            },
            None => {
                break;
            },
        }

        if nest_level >= 4 && !exploding {
            let (_, remaining) = sn_num.split_at(i);
            if pair_regex.is_match(remaining) {
                let (left, right) = get_pair_vals(remaining.to_string());

                if nums.len() != 0 {
                    let num_to_left = nums.pop().unwrap();
                    nums.push(num_to_left + left);
                }

                exploding = true;
                add_right = right;

                fstring.push('0');
                loop {
                    if char_iter.next().unwrap() == ']' {
                        break;
                    }
                }
                continue;
            }
        }

        match c {
            '[' => {
                fstring.push('[');
                nest_level += 1;
            },
            ']' => {
                if num_len != 0 {
                    fstring.push('$');
                    nums.push(num_val + add_right);
                    if exploding { // to just add it once
                        add_right = 0;
                    }
                    num_val = 0;
                    num_len = 0;
                }
                fstring.push(']');
                nest_level -= 1;
            },
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                let c_num = u64::from_str_radix(&c.to_string(), 10).unwrap();
                if num_len == 0 { // first number
                    num_val = c_num;
                } else {
                    num_val = num_val * 10 + c_num;
                }
                num_len += 1;
            },
            ',' => {
                if num_len != 0 {
                    fstring.push('$');
                    nums.push(num_val + add_right);
                    if exploding { // to just add it once
                        add_right = 0;
                    }
                    num_val = 0;
                    num_len = 0;
                }
                fstring.push(',');
            }
            _ => {
                panic!("Unexpected character: {}", c);
            }
        }
        i += 1;
    }

    let output = fmtstr(fstring, nums, '$');

    if exploding {
        if DEBUG { println!("after explode: {}", output); }
        return Some(output)
    }
    return None;
}

fn fmtstr(template: String, nums: Vec<u64>, magic_ch: char) -> String {
    let mut output = String::from("");
    let mut num_iter = nums.into_iter();
    for c in template.chars() {
        if c == magic_ch {
            output.push_str(&num_iter.next().unwrap().to_string());
        } else {
            output.push(c);
        }
    }
    return output;
}

fn get_pair_vals(input: String) -> (u64, u64) {
    let mut left = 0;
    let mut right = 0;
    let mut left_complete = false;

    for c in input.chars() {
        match c {
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                let c_num = u64::from_str_radix(&c.to_string(), 10).unwrap();
                if left_complete {
                    right = right*10 + c_num;
                } else {
                    left = left*10 + c_num;
                }
            },
            ',' => {
                left_complete = true;
            },
            ']' => {
                return (left, right);
            }
            _ => {
                // ignore
            }
        }
    }

    panic!("Could not get pair vals {}", input);
}

fn try_split(sn_num: String) -> Option<String> {
    let split_regex = Regex::new(r"\d\d+").unwrap();

    match split_regex.find(&sn_num) {
        Some(m) => {
            let num = u64::from_str_radix(&m.as_str(), 10).unwrap();
            let left = num/2;
            let right = num - left;
            let pair = format!("[{},{}]", left, right);

            let ans = split_regex.replace(&sn_num, pair).to_string();
            if DEBUG { println!("after split:   {}", ans); }
            return Some(ans);
        },
        None => {
            if DEBUG { println!("Done"); }
            return None;
        }
    }
}

fn magnitude(sn_num: String) -> String {
    let pair_regex = Regex::new(r"\[\d+,\d+\]").unwrap();
    let mut sn = sn_num.clone();    
    
    loop {
        match pair_regex.find(&sn) {
            Some(m) => {
                let raw_pair = m.as_str();
                let (left, right) = get_pair_vals(raw_pair.to_string());
                let mag = left*3 + right*2;
                sn = pair_regex.replace(&sn, mag.to_string()).to_string();
            },
            None => {
                return sn;
            }
        }
    }
}

fn read_data(path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut v = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(b';')
        .has_headers(false)
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        for item in record.into_iter() {
            v.push(item.to_string());
        }
    }

    return Ok(v);
}
