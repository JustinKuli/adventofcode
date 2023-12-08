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
    let mut cum_sum_more = 0;

    let mut grid = digits(data)?;
    for i in 0..100 {
        let (new_grid, flashes) = p1_step(grid.clone());
        grid = new_grid;
        cum_sum_more += flashes;
    }

    return Ok(cum_sum_more.to_string());
}

fn p1_step(inp: Vec<Vec<i64>>) -> (Vec<Vec<i64>>, i64) {
    let mut out = inp.clone();

    for i in 0..10 {
        for j in 0..10 {
            let val = out.get_mut(i).unwrap().get_mut(j).unwrap();
            *val += 1;
        }
    }

    let mut cum_sum: i64 = 0;
    let (updated, mut sum) = propagate_flashes(out.clone());
    out = updated.clone();
    cum_sum += sum;
    while sum != 0 {
        let (updated, new_sum) = propagate_flashes(out.clone());
        out = updated.clone();
        cum_sum += new_sum;
        sum = new_sum;
    }

    for i in 0..10 {
        for j in 0..10 {
            let val = out.get_mut(i).unwrap().get_mut(j).unwrap();
            if val < &mut 0 {
                *val = 0
            }
        }
    }

    return (out, cum_sum);
}

fn propagate_flashes(inp: Vec<Vec<i64>>) -> (Vec<Vec<i64>>, i64) {
    let mut out = inp.clone();

    let mut sum = 0;
    for i in 0..10 {
        for j in 0..10 {
            let val = out.get_mut(i).unwrap().get_mut(j).unwrap();
            if val > &mut 9 {
                sum += 1;
                *val = -999999;
                for i_d in 0..3 {
                    if (i + i_d) < 1 {
                        continue;
                    }
                    match out.get_mut(i+i_d-1) {
                        Some(row) => {
                            for j_d in 0..3 {
                                if (j + j_d) < 1 {
                                    continue;
                                }
                                match row.get_mut(j+j_d-1) {
                                    Some(neighbor) => {
                                        *neighbor += 1;
                                    },
                                    None => {},
                                }
                            }
                        },
                        None =>{},
                    }
                }
            }
        }
    }

    return (out, sum);
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let mut grid = digits(data)?;
    let mut i = 1;
    loop {
        let (new_grid, flashes) = p1_step(grid.clone());
        grid = new_grid;
        if flashes == 100 {
            break;
        }
        i += 1;
    }

    return Ok(i.to_string());
}

fn digits(data: Vec<csv::StringRecord>) -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    let mut v = Vec::new();

    for line in data {
        let mut vline = Vec::new();
        for c in line.as_slice().chars() {
            match c {
                '0' => vline.push(0),
                '1' => vline.push(1),
                '2' => vline.push(2),
                '3' => vline.push(3),
                '4' => vline.push(4),
                '5' => vline.push(5),
                '6' => vline.push(6),
                '7' => vline.push(7),
                '8' => vline.push(8),
                '9' => vline.push(9),
                _ => return Err("invalid char".into())
            }
        }
        v.push(vline);
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
