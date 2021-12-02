fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    part_one(data.clone());
    part_two(data.clone());
}

fn part_one(data: Vec<csv::StringRecord>) {
    let acc = 0;
    let mut v = Vec::new();

    for line in data {
        for item in line.into_iter() {
            let val = str::parse::<i32>(item)
                .expect("Or else");
            v.push(val)
        }
    }

    println!("part one: {}", acc)
}

fn part_two(data: Vec<csv::StringRecord>) {
    let acc = 0;

    println!("part two: {}", acc)
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
