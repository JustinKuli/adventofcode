#![allow(unused_variables)]

use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let data = read_array(String::from("data.txt"))
        .expect("Failed to get data");

    let ans1 = solve_puzzle(data.clone(), 100).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = solve_puzzle(data.clone(), 500).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

const GRID_SIZE: usize = 500;
const MINI_GRID_SIZE: usize = 100;

fn solve_puzzle(data: CaveGrid, size: i32) -> Result<String, Box<dyn Error>> {
    let mut to_visit = ToVisit{map: HashMap::new()};
    let mut visited = HashSet::<Point>::new();
    let mut curr = ValuedPoint{
        p: Point{x: 0, y: 0},
        val: 0,
    };

    while !(curr.p.x == size-1 && curr.p.y == size-1) {
        //println!("Visiting x={}, y={}, val={}", curr.p.x, curr.p.y, curr.val);
        let adj_points = data.adj(&curr.p);
        for adj in adj_points {
            if adj.p.x >= size || adj.p.y >= size {
                continue;
            }

            if !visited.contains(&adj.p) {
                let new_cost = adj.val + curr.val;
                match to_visit.map.get(&adj.p) {
                    Some(prev_cost) => {
                        if new_cost < *prev_cost {
                            to_visit.map.insert(adj.p.clone(), new_cost);
                        }
                    },
                    None => {
                        to_visit.map.insert(adj.p.clone(), new_cost);
                    },
                }
            }
        }
        visited.insert(curr.p.clone());
    
        curr = to_visit.next(curr.val);    
    }

    return Ok(curr.val.to_string());
}

struct ToVisit {
    map: HashMap<Point, i32>,
}

impl ToVisit {
    fn next(&mut self, last_val: i32) -> ValuedPoint {
        let mut smallest = i32::MAX;
        let mut point = Point{x: -1, y: -1};

        for (p, val) in self.map.iter() {
            if *val == last_val {
                smallest = *val;
                point = p.clone();
                break;
            }
            if *val < smallest {
                smallest = *val;
                point = p.clone();
            }
        }

        if point.x == -1 && point.y == -1 {
            panic!("Could not get next item from ToVisit?");
        }

        self.map.remove(&point);

        return ValuedPoint{
            p: point,
            val: smallest,
        }
    }
}

#[derive(Debug,Clone,Hash)]
struct ValuedPoint {
    p: Point,
    val: i32,
}
impl PartialEq for ValuedPoint {
    fn eq(&self, other: &Self) -> bool {
        return self.p.x == other.p.x && self.p.y == other.p.y;
    }
}
impl Eq for ValuedPoint {}

#[derive(Debug,Clone,Hash)]
struct Point {
    x: i32,
    y: i32,
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}
impl Eq for Point {}

#[derive(Debug,Clone)]
struct CaveGrid {
    arr: [[i32; GRID_SIZE]; GRID_SIZE],
}

impl CaveGrid {
    fn get(&self, p: &Point) -> i32 {
        let mut i = p.x;
        let mut j = p.y;
        if i < 0 || i > (GRID_SIZE as i32 - 1) || j < 0 || j > (GRID_SIZE as i32 - 1) {
            return i32::MAX/2;
        }
        if i < (MINI_GRID_SIZE as i32) && j < (MINI_GRID_SIZE as i32) {
            return self.arr[i as usize][j as usize];
        }

        let mut inc = 0;
        while i >= MINI_GRID_SIZE as i32 {
            inc += 1;
            i -= MINI_GRID_SIZE as i32;
        }
        while j >= MINI_GRID_SIZE as i32 {
            inc += 1;
            j -= MINI_GRID_SIZE as i32;
        }

        let mut val = self.arr[i as usize][j as usize];

        for _ in 0..inc {
            val += 1;
            if val == 10 {
                val = 1;
            }
        }

        return val;
    }

    fn adj(&self, point: &Point) -> Vec<ValuedPoint> {
        let mut v = Vec::new();

        let p1 = Point{x: point.x-1, y: point.y};
        if p1.x >= 0 && p1.x < (GRID_SIZE as i32) && p1.y >= 0 && p1.y < (GRID_SIZE as i32) {
            let vp1 = ValuedPoint{
                p: p1.clone(),
                val: self.get(&p1),
            };
            v.push(vp1);
        }

        let p2 = Point{x: point.x+1, y: point.y};
        if p2.x >= 0 && p2.x < (GRID_SIZE as i32) && p2.y >= 0 && p2.y < (GRID_SIZE as i32) {
            let vp2 = ValuedPoint{
                p: p2.clone(),
                val: self.get(&p2),
            };
            v.push(vp2);
        }

        let p3 = Point{x: point.x, y: point.y-1};
        if p3.x >= 0 && p3.x < (GRID_SIZE as i32) && p3.y >= 0 && p3.y < (GRID_SIZE as i32) {
            let vp3 = ValuedPoint{
                p: p3.clone(),
                val: self.get(&p3),
            };
            v.push(vp3);
        }

        let p4 = Point{x: point.x, y: point.y+1};
        if p4.x >= 0 && p4.x < (GRID_SIZE as i32) && p4.y >= 0 && p4.y < (GRID_SIZE as i32) {
            let vp4 = ValuedPoint{
                p: p4.clone(),
                val: self.get(&p4),
            };
            v.push(vp4);
        }

        return v;
    }
}

fn read_array(path: String) -> Result<CaveGrid, Box<dyn Error>> {
    let mut grid = CaveGrid{arr: [[0i32; GRID_SIZE]; GRID_SIZE]};

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(path)?;

    let mut i = 0;
    for line in rdr.records() {
        let record = line?;
        match record.into_iter().next() {
            None => {
                return Err("bad".into())
            }
            Some(full_line) => {
                let mut j = 0;
                for c in full_line.chars() {
                    grid.arr[i][j] = i32::from_str_radix(&c.to_string(), 10)?;
                    j += 1;
                }
            }
        }
        i += 1;
    }

    return Ok(grid);
}
