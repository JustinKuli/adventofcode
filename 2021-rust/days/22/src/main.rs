#![allow(unused_variables)]

use std::error::Error;
use std::collections::HashSet;
use std::cmp;

fn main() {
    let data = read_data(String::from("formatted.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(data.clone()).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

const DEBUG: bool = false;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn part_one(data: Vec<Step>) -> Result<String, Box<dyn Error>> {
    let mut cubes_on = HashSet::new();

    let mut i = 0;
    let step_num = data.len();
    for step in data.iter() {
        i += 1;
        if DEBUG { println!("Step {}/{}", i, step_num); }
        let mut x = step.x_start;
        while x <= step.x_end {
            if x < -50 || x > 50 {
                x += 1;
                continue;
            }
            let mut y = step.y_start;
            while y <= step.y_end {
                if y < -50 || y > 50 {
                    y += 1;
                    continue;
                }
                let mut z = step.z_start;
                while z <= step.z_end {
                    if z < -50 || z > 50 {
                        z += 1;
                        continue;
                    }

                    let p = Point{x:x, y:y, z:z};

                    if step.on {
                        cubes_on.insert(p);
                    } else {
                        cubes_on.remove(&p);
                    }

                    z += 1;
                }
                y += 1;
            }
            x += 1;
        }
    }

    let ans = cubes_on.len();

    return Ok(ans.to_string());
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Cuboid {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    z_start: i32,
    z_end: i32,
}

impl Cuboid {
    fn overlaps(&self, other: &Cuboid) -> bool {
        if !(self.x_start <= other.x_end && other.x_start <= self.x_end) {
            return false; // no overlap in x.
        }

        if !(self.y_start <= other.y_end && other.y_start <= self.y_end) {
            return false; // no overlap in y.
        }

        if !(self.z_start <= other.z_end && other.z_start <= self.z_end) {
            return false; // no overlap in z.
        }

        return true;
    }

    fn foobar(&self, x1: i32, x2: i32, y1: i32, y2: i32, z1: i32, z2: i32) -> Option<Cuboid> {
        match one_d_overlap(self.x_start, self.x_end, x1, x2) {
            Some(xs) => {
                match one_d_overlap(self.y_start, self.y_end, y1, y2) {
                    Some(ys) => {
                        match one_d_overlap(self.z_start, self.z_end, z1, z2) {
                            Some(zs) => {
                                return Some(Cuboid{
                                    x_start: xs.0,
                                    x_end: xs.1,
                                    y_start: ys.0,
                                    y_end: ys.1,
                                    z_start: zs.0,
                                    z_end: zs.1,
                                });
                            }
                            None => {},
                        }
                    },
                    None => {},
                }
            },
            None => {},
        }

        return None;
    }

    fn subtract(&self, other: &Cuboid) -> Vec<Cuboid> {
        let mut v = Vec::new();

        // 6 "faces"
        {
            match self.foobar(other.x_start, other.x_end, other.y_start, other.y_end, other.z_end+1, i32::MAX) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: face 1"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_start, other.x_end, other.y_start, other.y_end, i32::MIN, other.z_start-1) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: face 2"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_start, other.x_end, other.y_end+1, i32::MAX, other.z_start, other.z_end) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: face 3"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_start, other.x_end, i32::MIN, other.y_start-1, other.z_start, other.z_end) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: face 4"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_end+1, i32::MAX, other.y_start, other.y_end, other.z_start, other.z_end) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: face 5"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(i32::MIN, other.x_start-1, other.y_start, other.y_end, other.z_start, other.z_end) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: face 6"); }
                    v.push(c);
                }
                None => {}
            }
        }

        // 8 "corners"
        {
            match self.foobar(i32::MIN, other.x_start-1, i32::MIN, other.y_start-1, i32::MIN, other.z_start-1) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: corner 1"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(i32::MIN, other.x_start-1, other.y_end+1, i32::MAX, i32::MIN, other.z_start-1) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: corner 2"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(i32::MIN, other.x_start-1, i32::MIN, other.y_start-1, other.z_end+1, i32::MAX) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: corner 3"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(i32::MIN, other.x_start-1, other.y_end+1, i32::MAX, other.z_end+1, i32::MAX) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: corner 4"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_end+1, i32::MAX, i32::MIN, other.y_start-1, i32::MIN, other.z_start-1) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: corner 5"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_end+1, i32::MAX, other.y_end+1, i32::MAX, i32::MIN, other.z_start-1) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: corner 6"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_end+1, i32::MAX, i32::MIN, other.y_start-1, other.z_end+1, i32::MAX) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: corner 7"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_end+1, i32::MAX, other.y_end+1, i32::MAX, other.z_end+1, i32::MAX) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: corner 8"); }
                    v.push(c);
                }
                None => {}
            }
        }

        // 12 "edges"
        {
            match self.foobar(i32::MIN, other.x_start-1, other.y_start, other.y_end, i32::MIN, other.z_start-1) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 1"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(i32::MIN, other.x_start-1, other.y_start, other.y_end, other.z_end+1, i32::MAX) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 2"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_end+1, i32::MAX, other.y_start, other.y_end, i32::MIN, other.z_start-1) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 3"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_end+1, i32::MAX, other.y_start, other.y_end, other.z_end+1, i32::MAX) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 4"); }
                    v.push(c);
                }
                None => {}
            }

            match self.foobar(other.x_start, other.x_end, i32::MIN, other.y_start-1, i32::MIN, other.z_start-1) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 5"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_start, other.x_end, i32::MIN, other.y_start-1, other.z_end+1, i32::MAX) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 6"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_start, other.x_end, other.y_end+1, i32::MAX, i32::MIN, other.z_start-1) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 7"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_start, other.x_end, other.y_end+1, i32::MAX, other.z_end+1, i32::MAX) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 8"); }
                    v.push(c);
                }
                None => {}
            }

            match self.foobar(i32::MIN, other.x_start-1, i32::MIN, other.y_start-1, other.z_start, other.z_end) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 9"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(i32::MIN, other.x_start-1, other.y_end+1, i32::MAX, other.z_start, other.z_end) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 10"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_end+1, i32::MAX, i32::MIN, other.y_start-1, other.z_start, other.z_end) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 11"); }
                    v.push(c);
                }
                None => {}
            }
            match self.foobar(other.x_end+1, i32::MAX, other.y_end+1, i32::MAX, other.z_start, other.z_end) {
                Some(c) => {
                    if DEBUG { println!("subtract case added cuboid: edge 12"); }
                    v.push(c);
                }
                None => {}
            }
        }

        return v;
    }

    fn size(&self) -> u128 {
        let x_size = (1 + self.x_end - self.x_start) as u128;
        let y_size = (1 + self.y_end - self.y_start) as u128;
        let z_size = (1 + self.z_end - self.z_start) as u128;
        return x_size * y_size * z_size;
    }
}

fn one_d_overlap(x1: i32, x2: i32, y1: i32, y2: i32) -> Option<(i32, i32)> {
    let start = cmp::max(x1, y1);
    let end = cmp::min(x2, y2);
    if start <= end {
        return Some((start, end));
    }
    return None;
}

fn part_two(data: Vec<Step>) -> Result<String, Box<dyn Error>> {
    let mut cuboids = HashSet::<Cuboid>::new();

    let mut i = 0;
    let step_num = data.len();
    for step in data.iter() {
        i += 1;
        if DEBUG { println!("Start Step {}/{}", i, step_num); }
        let mut acc = 0;
        for c in &cuboids {
            acc += c.size();
            // println!("{:?}", c);
        }    
        if DEBUG { println!("total lit: {}", acc); }

        let new_c = Cuboid{
            x_start: step.x_start,
            x_end: step.x_end,
            y_start: step.y_start,
            y_end: step.y_end,
            z_start: step.z_start,
            z_end: step.z_end,
        };

        for c in cuboids.clone().iter() {
            if new_c.overlaps(&c) {
                // println!("{:?} overlaps {:?}", new_c, c);
                cuboids.remove(&c);
                // println!("removed 1 from cuboids, len={}", cuboids.len());
                let mut added = 0;
                for split_c in c.subtract(&new_c).iter() {
                    added += 1;
                    // println!("inserting {:?}", split_c);
                    cuboids.insert(split_c.clone());
                }
                // println!("added {} to cuboids, len={}", added, cuboids.len());
            }
        }

        if step.on {
            // println!("Step=on, adding cuboid {:?}", new_c);
            cuboids.insert(new_c.clone());
            // println!("new len={}", cuboids.len());
        }
    }

    let mut sum = 0;
    for c in cuboids {
        //println!("{:?}", c);
        sum += c.size();
    }

    return Ok(sum.to_string());
}

#[derive(Clone, Debug)]
struct Step {
    on: bool,
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    z_start: i32,
    z_end: i32,
}

fn read_data(path: String) -> Result<Vec<Step>, Box<dyn Error>> {
    let mut v = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(path)?;

    for result in rdr.records() {
        let line = result?;
        let mut line_iter = line.iter();
        let on = line_iter.next().unwrap() == "on";
        let x_start = i32::from_str_radix(line_iter.next().unwrap(), 10)?;
        let x_end = i32::from_str_radix(line_iter.next().unwrap(), 10)?;
        let y_start = i32::from_str_radix(line_iter.next().unwrap(), 10)?;
        let y_end = i32::from_str_radix(line_iter.next().unwrap(), 10)?;
        let z_start = i32::from_str_radix(line_iter.next().unwrap(), 10)?;
        let z_end = i32::from_str_radix(line_iter.next().unwrap(), 10)?;

        v.push(Step{
            on: on,
            x_start: x_start,
            x_end: x_end,
            y_start: y_start,
            y_end: y_end,
            z_start: z_start,
            z_end: z_end,
        })
    }

    return Ok(v);
}
