#![allow(unused_variables)]

use std::error::Error;
use std::collections::HashSet;


fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);
    // 1169 is too high

    let ans2 = part_two(data.clone()).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

fn part_one(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let mut plot = plot_points(data)?;
    plot = fold_x(plot.clone(), 655);
    
    return Ok(plot.len().to_string());
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let mut plot = plot_points(data)?;
    plot = fold_x(plot.clone(), 655);
    plot = fold_y(plot.clone(), 447);
    plot = fold_x(plot.clone(), 327);
    plot = fold_y(plot.clone(), 223);
    plot = fold_x(plot.clone(), 163);
    plot = fold_y(plot.clone(), 111);
    plot = fold_x(plot.clone(), 81);
    plot = fold_y(plot.clone(), 55);
    plot = fold_x(plot.clone(), 40);
    plot = fold_y(plot.clone(), 27);
    plot = fold_y(plot.clone(), 13);
    plot = fold_y(plot.clone(), 6);

    for y in 0..6 {
        for x in 0..40 {
            let p = Point{x:x, y:y};
            if plot.contains(&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    return Ok(String::from(""));
}

fn fold_x(plot: HashSet<Point>, fold:i32) -> HashSet<Point> {
    let mut new_plot = HashSet::new();
    for p in plot.into_iter() {
        if p.x > fold {
            let new_x = fold - (p.x - fold);
            let new_p = Point {
                x: new_x,
                y: p.y,
            };
            new_plot.insert(new_p);
        } else {
            new_plot.insert(p);
        }
    }
    return new_plot;
}

fn fold_y(plot: HashSet<Point>, fold:i32) -> HashSet<Point> {
    let mut new_plot = HashSet::new();
    for p in plot.into_iter() {
        if p.y > fold {
            let new_y = fold - (p.y - fold);
            let new_p = Point {
                y: new_y,
                x: p.x,
            };
            new_plot.insert(new_p);
        } else {
            new_plot.insert(p);
        }
    }
    return new_plot;
}


#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn plot_points(data: Vec<csv::StringRecord>) -> Result<HashSet<Point>, Box<dyn Error>> {
    let mut plot = HashSet::<Point>::new();

    for line in data {
        let mut iter = line.into_iter();
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let p = Point{
            x: i32::from_str_radix(x, 10)?,
            y: i32::from_str_radix(y, 10)?,
        };
        plot.insert(p);
    }

    return Ok(plot);
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
