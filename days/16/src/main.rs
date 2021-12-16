#![allow(unused_variables)]

use std::error::Error;

fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");
    // we'll want to reverse it later, so that `pop` will get the "first" bit

    let ans1 = part_one(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1); // 871

    let ans2 = part_two(data.clone()).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2); // 68703010504
}

fn part_one(data: Vec<bool>) -> Result<String, Box<dyn Error>> {
    let mut mypacket = data.clone();

    let ans = recursive_version_sum(&mut mypacket);

    return Ok(ans.to_string());
}

fn recursive_version_sum(packet: &mut Vec<bool>) -> i32 {
    if packet.len() == 0 {
        return 0;
    }

    let mut version = 0;
    for _ in 0..3 {
        version = version << 1;
        if packet.pop().unwrap() {
            version += 1;
        }
    }

    let mut ptype = 0;
    for _ in 0..3 {
        ptype = ptype << 1;
        if packet.pop().unwrap() {
            ptype += 1;
        }
    }

    match ptype {
        4 => {
            let mut val: u64 = 0;
            let mut more = true;
            while more {
                val = val << 4;
                more = packet.pop().unwrap();
                for _ in 0..4 {
                    val = val << 1;
                    if packet.pop().unwrap() {
                        val += 1;
                    }
                }
            }
            return version;
        },
        _ => { // operator packet
            if packet.pop().unwrap() {
                let mut length = 0; //11 
                for _ in 0..11 {
                    length = length << 1;
                    if packet.pop().unwrap() {
                        length += 1;
                    }
                }

                for _ in 0..length {
                    version += recursive_version_sum(packet);
                }
                return version;
            } else {
                let mut length = 0; // 15
                for _ in 0..15 {
                    length = length << 1;
                    if packet.pop().unwrap() {
                        length += 1;
                    }
                }
            
                let start_len = packet.len();
                let end_len = start_len - length;
                while packet.len() > end_len {
                    let packets_read = start_len - packet.len();
                    version += recursive_version_sum(packet);
                }
                return version;
            }
        }
    }
}

fn part_two(data: Vec<bool>) -> Result<String, Box<dyn Error>> {
    let mut mypacket = data.clone();

    let ans = recursive_evaluate(&mut mypacket);

    return Ok(ans.to_string());
}

fn recursive_evaluate(packet: &mut Vec<bool>) -> i64 {
    if packet.len() == 0 {
        return 0;
    }

    let mut version = 0;
    for _ in 0..3 {
        version = version << 1;
        if packet.pop().unwrap() {
            version += 1;
        }
    }

    let mut ptype = 0;
    for _ in 0..3 {
        ptype = ptype << 1;
        if packet.pop().unwrap() {
            ptype += 1;
        }
    }

    let mut subvals = Vec::new();

    match ptype {
        4 => {
            let mut val: i64 = 0;
            let mut more = true;
            while more {
                more = packet.pop().unwrap();
                for _ in 0..4 {
                    val = val << 1;
                    if packet.pop().unwrap() {
                        val += 1;
                    }
                }
            }
            return val;
        },
        _ => { // operator packet
            if packet.pop().unwrap() {
                let mut length = 0;
                for _ in 0..11 {
                    length = length << 1;
                    if packet.pop().unwrap() {
                        length += 1;
                    }
                }

                for _ in 0..length {
                    subvals.push(recursive_evaluate(packet));
                }
            } else {
                let mut length = 0;
                for _ in 0..15 {
                    length = length << 1;
                    if packet.pop().unwrap() {
                        length += 1;
                    }
                }

                let start_len = packet.len();
                let end_len = start_len - length;
                while packet.len() > end_len {
                    let packets_read = start_len - packet.len();
                    subvals.push(recursive_evaluate(packet));
                }
            }
        }
    }

    match ptype {
        0 => { // sum
            let mut sum = 0;
            for val in subvals {
                sum += val;
            }
            return sum;
        },
        1 => { // product
            let mut prod = 1;
            for val in subvals {
                prod *= val;
            }
            return prod;
        },
        2 => { // minimum
            let mut min = i64::MAX;
            for val in subvals {
                if val < min {
                    min = val;
                }
            }
            return min;
        },
        3 => { // maximum
            let mut max = -1;
            for val in subvals {
                if val > max {
                    max = val;
                }
            }
            return max;
        }
        5 => { // greater than
            if subvals.get(0).unwrap() > subvals.get(1).unwrap() {
                return 1;
            }
            return 0;
        },
        6 => { // less than
            if subvals.get(0).unwrap() < subvals.get(1).unwrap() {
                return 1;
            }
            return 0;
        },
        7 => { // equal to
            if subvals.get(0).unwrap() == subvals.get(1).unwrap() {
                return 1;
            }
            return 0;
        },
        _ => {
            panic!("invalid ptype {}", ptype);
        }
    }
}

fn read_data(path: String) -> Result<Vec<bool>, Box<dyn Error>> {
    let mut hex = String::from("");

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        match record.into_iter().next() {
            None => {
                return Err("bad".into())
            },
            Some(line) => {
                hex = line.to_string();
            },
        }
    }

    let mut bin = String::from("");
    for c in hex.chars() {
        match c {
            '0' => {
                bin.push_str("0000");
            },
            '1' => {
                bin.push_str("0001");
            },
            '2' => {
                bin.push_str("0010");
            },
            '3' => {
                bin.push_str("0011");
            },
            '4' => {
                bin.push_str("0100");
            },
            '5' => {
                bin.push_str("0101");
            },
            '6' => {
                bin.push_str("0110");
            },
            '7' => {
                bin.push_str("0111");
            },
            '8' => {
                bin.push_str("1000");
            },
            '9' => {
                bin.push_str("1001");
            },
            'A' => {
                bin.push_str("1010");
            },
            'B' => {
                bin.push_str("1011");
            },
            'C' => {
                bin.push_str("1100");
            },
            'D' => {
                bin.push_str("1101");
            },
            'E' => {
                bin.push_str("1110");
            },
            'F' => {
                bin.push_str("1111");
            },
            _ => {
                return Err("real bad".into())
            }
        }
    }

    let mut data = Vec::new();
    // reverse it, so that `pop` will get the "first" bit
    for c in bin.chars().rev() {
        match c {
            '0' => {
                data.push(false);
            },
            '1' => {
                data.push(true);
            },
            _ => {
                return Err("really bad, like how is it this bad".into())
            }
        }
    }

    return Ok(data);
}
