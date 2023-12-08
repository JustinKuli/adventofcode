use std::error::Error;
// use std::collections::HashSet;
// use std::collections::HashMap;

fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(&data).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(&data).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

fn part_one(data: &Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let mut acc = i32::MIN;

    // sample code finds the largest value in the data
    let items = single_stream(data, 10)?;
    for thing in items {
        if thing > acc {
            acc = thing;
        }
    }
    
    return Ok(acc.to_string());
}

fn part_two(_data: &Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    return Ok(String::from(""));
}

fn single_stream(data: &Vec<csv::StringRecord>, radix: u32) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut v = Vec::new();

    for line in data {
        for item in line.into_iter() {
            let n = i32::from_str_radix(item, radix)?;
            v.push(n);
            // for strings:
            // v.push(item.to_string());
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
