#![allow(unused_variables)]

use std::error::Error;

fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let length = first_word_len(data.clone()).expect("Failed to get length of first word");
    let strings = ss_string(data).expect("Failed to convert to strings");

    let ans1 = part_one(strings.clone(), length).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(strings.clone(), length).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

// bit_balance returns how many more ones are in the data at the specified index
// compared to the number of zeroes at that index. So, if there are more ones,
// it will return a positive number, if they are equal, it will return 0, and 
// if there are more zeroes, it will return a negative number.
fn bit_balance(data: Vec<String>, index: usize) -> Result<i32, &'static str> {
    let mut acc = 0;

    for word in data {
        let b = word.into_bytes()[index];
        if b == 48 {
            acc -= 1;
        } else if b == 49 {
            acc += 1;
        } else {
            return Err("Unexpected byte");
        }
    }

    return Ok(acc);
}

fn part_one(data: Vec<String>, length: usize) -> Result<String, Box<dyn Error>> {
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..length {
        let bb = bit_balance(data.clone(), i)?;
        // bitshifts to add powers of 2
        if bb > 0 {
            gamma += 1 << (length-i-1);
        } else {
            epsilon += 1 << (length-i-1);
        }
    }

    let acc = gamma * epsilon;
    // println!("gamma: {}, epsilon: {}", gamma, epsilon);
    return Ok(acc.to_string());
}


fn part_two(data: Vec<String>, length: usize) -> Result<String, Box<dyn Error>> {
    let mut pos_ox_vals = data.clone();
    let mut pos_co2_vals = data.clone();
    let mut ox_val = 0;
    let mut co2_val = 0;

    'oxloop: for i in 0..length {
        // end loop early if only one possibility left
        let check_vec = pos_ox_vals.clone();
        if check_vec.len() == 1 {
            let ox_str = check_vec.get(0).unwrap();
            ox_val = i32::from_str_radix(ox_str, 2)?;
            break 'oxloop;
        }

        let bb = bit_balance(pos_ox_vals.clone(), i)?;

        if bb >= 0 {
            // keep ones
            pos_ox_vals = pos_ox_vals.into_iter()
                .filter(|val| val.clone().into_bytes()[i] == 49)
                .collect();
        } else {
            // keep zeroes
            pos_ox_vals = pos_ox_vals.into_iter()
                .filter(|val| val.clone().into_bytes()[i] == 48)
                .collect();
        }
    }

    if ox_val == 0 {
        if pos_ox_vals.len() == 1 {
            let ox_str = pos_ox_vals.get(0).unwrap();
            ox_val = i32::from_str_radix(ox_str, 2)?;
        } else {
            return Err("Exited oxloop without an ox_val".into())
        }
    }

    'co2loop: for i in 0..length {
        // end loop early if only one possibility left
        let check_vec = pos_co2_vals.clone();
        if check_vec.len() == 1 {
            let co2_str = check_vec.get(0).unwrap();
            co2_val = i32::from_str_radix(co2_str, 2)?;
            break 'co2loop;
        }

        let bb = bit_balance(pos_co2_vals.clone(), i)?;

        if bb >= 0 {
            // keep zeroes
            pos_co2_vals = pos_co2_vals.into_iter()
                .filter(|val| val.clone().into_bytes()[i] == 48)
                .collect();
        } else {
            // keep ones
            pos_co2_vals = pos_co2_vals.into_iter()
                .filter(|val| val.clone().into_bytes()[i] == 49)
                .collect();
        }
    }

    if co2_val == 0 {
        if pos_co2_vals.len() == 1 {
            let co2_str = pos_co2_vals.get(0).unwrap();
            co2_val = i32::from_str_radix(co2_str, 2)?;
        } else {
            return Err("Exited co2loop without an co2_val".into())
        }
    }

    println!("ox: {}, co2: {}", ox_val, co2_val);
    let acc = ox_val * co2_val;
    return Ok(acc.to_string());
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
