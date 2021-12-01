fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let vals = data_to_vec(data);

    let ans1 = part_one(vals.clone());
    let ans2 = part_two(vals.clone());

    println!("part one: {}", ans1);
    println!("part two: {}", ans2);
}

fn data_to_vec(data: Vec<csv::StringRecord>) -> Vec<i32> {
    let mut v = Vec::new();

    for line in data {
        for item in line.into_iter() {
            let val = str::parse::<i32>(item)
                .expect("Or else");
            v.push(val)
        }
    }

    return v;
}

fn part_one(vals: Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut count = -1; // "first" increment doesn't count
    for v in vals {
        if v > prev {
            count += 1;
        }
        prev = v;
    }
    return count
}

fn part_two(vals: Vec<i32>) -> i32 {
    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    let mut i = 0;
    let mut count = -3; // first three increments don't count

    for v in vals {
        i += 1;
        if i % 3 == 1 {
            if (two + v) > one {
                count += 1;
            } else {
            }
            one = 0;
        } else if i % 3 == 2 {
            if (three + v) > two {
                count += 1;
            } else {
            }
            two = 0;
        } else {
            if (one + v) > three {
                count += 1;
            }else {
            }
            three = 0;
        }

        one += v;
        two += v;
        three += v;
    }

    return count
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
