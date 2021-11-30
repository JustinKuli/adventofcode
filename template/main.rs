fn main() {
    let data = read_data(String::from("formatted.txt"))
        .expect("Failed to get data");

    dbg_print_data(data)
}

fn dbg_print_data(data: Vec<csv::StringRecord>) {
    let mut i = 0;
    for line in data {
        for item in line.into_iter() {
            println!("Line: {} \t Item: {}", i, item)
        }
        i += 1;
    }
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
