use std::error::Error;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(&data).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(&data).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

const DEBUG: bool = true;

fn part_one(data: &Vec<Vec<Directive>>) -> Result<String, Box<dyn Error>> {
    let mut min_dist = i32::MAX;

    let p1 = trace_path(&data[0]);
    let p2 = trace_path(&data[1]);

    for p in p1 {
        if p2.contains(&p) {
            let dist = p.manh();
            if dist < min_dist {
                min_dist = dist;
            }
        }
    }
    
    return Ok(min_dist.to_string());
}

fn trace_path(directives: &Vec<Directive>) -> HashSet<Point> {
    let mut points = HashSet::new();

    let mut pos = Point{x:0, y:0};
    // don't put 0,0 into `points`... we don't care about that intersection
    for d in directives {
        let dist = d.distance as usize;
        match d.direction {
            Direction::Up => {
                for _ in 0..dist {
                    pos = Point{y: pos.y+1, ..pos};
                    points.insert(pos.clone());
                }
            }
            Direction::Down => {
                for _ in 0..dist {
                    pos = Point{y: pos.y-1, ..pos};
                    points.insert(pos.clone());
                }
            }
            Direction::Right => {
                for _ in 0..dist {
                    pos = Point{x: pos.x+1, ..pos};
                    points.insert(pos.clone());
                }
            }
            Direction::Left => {
                for _ in 0..dist {
                    pos = Point{x: pos.x-1, ..pos};
                    points.insert(pos.clone());
                }
            }
        }
    }

    return points;
}

#[derive(Hash,Eq,PartialEq,Debug,Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manh(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}

fn part_two(data: &Vec<Vec<Directive>>) -> Result<String, Box<dyn Error>> {
    let mut min_time = i32::MAX;

    let p1 = time_path(&data[0]);
    let p2 = time_path(&data[1]);

    for (loc, t1) in p1 {
        if let Some(t2) = p2.get(&loc) {
            let total_time = t1 + t2;
            if total_time < min_time {
                min_time = total_time;
            }
        }
    }
    
    return Ok(min_time.to_string());
}

fn time_path(directives: &Vec<Directive>) -> HashMap<Point, i32> {
    let mut points = HashMap::<Point, i32>::new();

    let mut pos = Point{x:0, y:0};
    let mut duration = 0;
    // don't put 0,0 into `points`... we don't care about that intersection
    for d in directives {
        for _ in 0..(d.distance as usize) {
            pos = match d.direction {
                Direction::Up => {
                    Point{y: pos.y+1, ..pos}
                }
                Direction::Down => {
                    Point{y: pos.y-1, ..pos}
                }
                Direction::Right => {
                    Point{x: pos.x+1, ..pos}
                }
                Direction::Left => {
                    Point{x: pos.x-1, ..pos}
                }
            };
            duration += 1;

            if let Some(prev) = points.insert(pos.clone(), duration) {
                // the old value definitely had a smaller duration, so put it back
                points.insert(pos.clone(), prev);
            }
        }
    }

    return points;
}

fn read_data(path: String) -> Result<Vec<Vec<Directive>>, Box<dyn Error>> {
    let mut data = Vec::<Vec<Directive>>::new();

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        let mut directives = Vec::<Directive>::new();

        for item in &record {
            let mut s = item.to_string();
            let rest = s.split_off(1);
            let dir = s.chars().next().unwrap();
            let dist = i32::from_str_radix(&rest, 10)?;

            match dir {
                'U' => {
                    directives.push(Directive{direction: Direction::Up, distance: dist});
                }
                'L' => {
                    directives.push(Directive{direction: Direction::Left, distance: dist});
                }
                'R' => {
                    directives.push(Directive{direction: Direction::Right, distance: dist});
                }
                'D' => {
                    directives.push(Directive{direction: Direction::Down, distance: dist});
                }
                _ => {
                    panic!("Unexpected direction {}", dir)
                }
            }
        }
        data.push(directives);
    }

    return Ok(data);
}

enum Direction {
    Up,
    Left,
    Right,
    Down
}

struct Directive{
    direction: Direction,
    distance: i32,
}
