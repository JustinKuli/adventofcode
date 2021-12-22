#![allow(unused_variables)]

use std::error::Error;

fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(&data).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(&data).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

fn part_one(data: &Grid) -> Result<String, Box<dyn Error>> {
    let g1 = data.enhance();
    let g2 = g1.enhance();
    
    return Ok(g2.count().to_string());
}

fn part_two(data: &Grid) -> Result<String, Box<dyn Error>> {
    let mut g = data.clone();

    for i in 0..50 {
        println!("Iteration {}", i);
        g = g.enhance();
    }

    return Ok(g.count().to_string());
}

#[derive(Debug,Clone)]
struct Grid {
    size: usize,
    grid: Vec<Vec<bool>>,
    default: bool,
    algorithm: Vec<bool>,
}

impl Grid {
    fn get(&self, i: i32, j: i32) -> bool {
        if i < 0 || j < 0 || (i as usize) >= self.grid.len() {
            return self.default;
        }
        let row = self.grid.get(i as usize).unwrap();
        if (j as usize) >= row.len() {
            return self.default
        }
        return *row.get(j as usize).unwrap();
    }

    fn sq_around(&self, i: i32, j: i32) -> usize {
        let mut val = 0;

        if self.get(i-1, j-1) {
            val += 1;
        }
        val = val << 1;

        if self.get(i-1, j) {
            val += 1;
        }
        val = val << 1;

        if self.get(i-1, j+1) {
            val += 1;
        }
        val = val << 1;

        if self.get(i, j-1) {
            val += 1;
        }
        val = val << 1;

        if self.get(i, j) {
            val += 1;
        }
        val = val << 1;

        if self.get(i, j+1) {
            val += 1;
        }
        val = val << 1;

        if self.get(i+1, j-1) {
            val += 1;
        }
        val = val << 1;

        if self.get(i+1, j) {
            val += 1;
        }
        val = val << 1;

        if self.get(i+1, j+1) {
            val += 1;
        }

        return val;
    }

    fn enhance(&self) -> Grid {
        let mut new_grid = Vec::new();
        let mut i = -1;
        while i <= self.size as i32 {
            let mut new_row = Vec::new();
            let mut j = -1;
            while j <= self.size as i32 {
                let val = self.sq_around(i,j);
                match self.algorithm.get(val) {
                    Some(b) => {
                        new_row.push(*b);
                    },
                    None => {
                        println!("Could not get {} from algorithm", val);
                        panic!("Bad");
                    }
                }
                // new_row.push(*self.algorithm.get(val).unwrap());
                j += 1;
            }
            new_grid.push(new_row);
            i += 1;
        }
    
        let mut new_default = self.default;
        if *self.algorithm.get(0).unwrap() {
            new_default = !self.default;
        }
    
        return Grid {
            size: self.size + 2,
            grid: new_grid,
            default: new_default,
            algorithm: self.algorithm.clone(),
        };
    }

    fn count(&self) -> u64 {
        let mut acc = 0;
        for row in self.grid.iter() {
            for item in row.iter() {
                if *item {
                    acc += 1;
                }
            }
        }
        return acc;
    }
}

fn read_data(path: String) -> Result<Grid, Box<dyn Error>> {
    let mut algorithm = Vec::new();
    let mut grid = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(path)?;

    let mut first = true;

    for result in rdr.records() {
        let record = result?;
        if first {
            for item in record.iter() {
                for c in item.chars() {
                    algorithm.push(c == '#');
                }
            }
            first = false;
            continue;
        }
        
        let mut row = Vec::new();
        for item in record.iter() {
            for c in item.chars() {
                row.push(c == '#');
            }
        }
        grid.push(row);
    }

    return Ok(Grid{
        size: grid.get(0).unwrap().len(),
        grid: grid,
        default: false,
        algorithm: algorithm,
    });
}
