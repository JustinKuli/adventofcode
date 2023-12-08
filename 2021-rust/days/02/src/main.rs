fn main() {
    let data = read_data(String::from("formatted.txt"))
        .expect("Failed to get data");

    part_one(data.clone());
    part_two(data.clone());
}

fn part_one(data: Vec<csv::StringRecord>) {
    let mut depth = 0;
    let mut dist = 0;

    for line in data {
        let direction = line.get(0).expect("couldn't get directive");
        let amount = line.get(1).expect("couldn't get amount");
        let delta = str::parse::<i32>(amount).expect("couldn't parse amount");
        if direction == "forward" {
            dist += delta;
        } else if direction == "down" {
            depth += delta;
        } else if direction == "up" {
            depth -= delta;
        } else {
            println!("Unexpected direction: {}", direction);
        }
    }    

    println!("Depth: {} , Distance: {} , Product: {}", depth, dist, depth*dist)
}

fn part_two(data: Vec<csv::StringRecord>) {
    let mut depth = 0;
    let mut dist = 0;
    let mut aim = 0;

    for line in data {
        let direction = line.get(0).expect("couldn't get directive");
        let amount = line.get(1).expect("couldn't get amount");
        let delta = str::parse::<i32>(amount).expect("couldn't parse amount");
        if direction == "forward" {
            dist += delta;
            depth += aim*delta;
        } else if direction == "down" {
            aim += delta;
        } else if direction == "up" {
            aim -= delta;
        } else {
            println!("Unexpected direction: {}", direction);
        }
    }    

    println!("Depth: {} , Distance: {} , Product: {}", depth, dist, depth*dist)
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
