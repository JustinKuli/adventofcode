fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    println!("part one: {}", part_one(data.clone()));
    println!("part two: {}", part_two(data.clone()));
}

fn get_first_word_len(data: Vec<csv::StringRecord>) -> usize {
    // need a helper function in order to handle the sample data,
    // since it is a different size from the puzzle
    // (otherwise it would be hardcoded)
    for line in data {
        for item in line.into_iter() {
            return item.len();
        }
    }
    return 0;
}

// bit_balance returns how many more ones are in the data at the specified bit
// compared to the number of zeroes at that bit. So, if there are more ones,
// it will return a positive number, if they are equal, it will return 0, and 
// if there are more zeroes, it will return a negative number.
fn bit_balance(data: Vec<csv::StringRecord>, bit: usize) -> Result<i32, &'static str> {
    let mut acc = 0;
    for line in data.clone() {
        for item in line.into_iter() {
            let b = item.as_bytes()[bit];
            if b == 48 {
                acc -= 1;
            } else if b == 49 {
                acc += 1;
            } else {
                return Err("Unexpected byte");
            }
        }
    }
    return Ok(acc);
}

fn part_one(data: Vec<csv::StringRecord>) -> i32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    let length = get_first_word_len(data.clone());

    for i in 0..length {
        let bb = bit_balance(data.clone(), i).unwrap();
        if bb > 0 {
            gamma += 1 << (length-i-1);
        } else {
            epsilon += 1 << (length-i-1);
        }
    }

    // println!("gamma {} epsilon {}", gamma, epsilon);
    return gamma * epsilon;
}

fn part_two(data: Vec<csv::StringRecord>) -> i32 {
    let length = get_first_word_len(data.clone());

    let mut pos_ox_vals = data.clone();
    let mut pos_co2_vals = data.clone();
    let mut ox_val = 0;
    let mut co2_val = 0;

    'oxloop: for i in 0..length {
        let check_vec = pos_ox_vals.clone();
        if check_vec.len() == 1 {
            let ox_str = check_vec.get(0).unwrap().get(0).unwrap();
            // str::parse::<i32>
            ox_val = i32::from_str_radix(ox_str, 2).unwrap();
            break 'oxloop;
        }

        let bb = bit_balance(pos_ox_vals.clone(), i).unwrap();

        let old_ox_vals = pos_ox_vals.clone();
        pos_ox_vals = Vec::new();

        if bb >= 0 {
            // keep ones
            for line in old_ox_vals {
                for item in line.into_iter() {
                    let b = item.as_bytes()[i];
                    if b == 49 {
                        pos_ox_vals.push(line.clone())
                    }
                }
            }
        } else {
            // keep zeroes
            for line in old_ox_vals {
                for item in line.into_iter() {
                    let b = item.as_bytes()[i];
                    if b == 48 {
                        pos_ox_vals.push(line.clone())
                    }
                }
            }
        }
    }

    if ox_val == 0 {
        if pos_ox_vals.len() == 1 {
            let ox_str = pos_ox_vals.get(0).unwrap().get(0).unwrap();
            ox_val = i32::from_str_radix(ox_str, 2).unwrap();
        } else {
            // If we get through all the bits and don't have exactly one
            // possible answer, then we have a problem, and print a warning.
            println!("ox warning");
        }
    }

    'co2loop: for i in 0..length {
        let check_vec = pos_co2_vals.clone();
        if check_vec.len() == 1 {
            let co2_str = check_vec.get(0).unwrap().get(0).unwrap();
            // str::parse::<i32>
            co2_val = i32::from_str_radix(co2_str, 2).unwrap();
            break 'co2loop;
        }

        let bb = bit_balance(pos_co2_vals.clone(), i).unwrap();

        let old_co2_vals = pos_co2_vals.clone();
        pos_co2_vals = Vec::new();

        if bb < 0 {
            // keep ones
            for line in old_co2_vals {
                for item in line.into_iter() {
                    let b = item.as_bytes()[i];
                    if b == 49 {
                        pos_co2_vals.push(line.clone())
                    }
                }
            }
        } else {
            // keep zeroes
            for line in old_co2_vals {
                for item in line.into_iter() {
                    let b = item.as_bytes()[i];
                    if b == 48 {
                        pos_co2_vals.push(line.clone())
                    }
                }
            }
        }
    }

    if co2_val == 0 {
        if pos_co2_vals.len() == 1 {
            let co2_str = pos_co2_vals.get(0).unwrap().get(0).unwrap();
            co2_val = i32::from_str_radix(co2_str,2).unwrap();
        } else {
            // If we get through all the bits and don't have exactly one
            // possible answer, then we have a problem, and print a warning.
            println!("co2 warning");
        }
    }

    println!("ox: {}, co2: {}", ox_val, co2_val);
    return ox_val*co2_val;
}

fn read_data(path: String) -> Result<Vec<csv::StringRecord>, Box<dyn std::error::Error>> {
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
