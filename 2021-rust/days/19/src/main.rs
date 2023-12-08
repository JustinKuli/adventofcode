#![allow(unused_variables)]

use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;

fn main() {
    let data = read_data(String::from("small-formatted.txt"))
        .expect("Failed to get data");

    let ans1 = puzzle(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);
}

const EPSILON: f64 = 0.000001; // for comparing floats.

fn puzzle(data: Vec<Scanner>) -> Result<String, Box<dyn Error>> {
    let mut found_beacons = HashSet::<Point>::new(); 
    let mut known_scanners = Vec::<Scanner>::new();
    let mut unknown_scanners = Vec::<Scanner>::new();

    let mut init_data_iter = data.iter();

    let mut first_scanner = init_data_iter.next().unwrap().clone();
    first_scanner.orientation = Quat {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
    };
    known_scanners.push(first_scanner);

    for s in init_data_iter {
        unknown_scanners.push(s.clone());
    }

    'unknown: while unknown_scanners.len() != 0 {
        let mut s2 = unknown_scanners.pop().unwrap().clone();

        for s1 in known_scanners.iter() {
            let dist_pairs = matching_distances(&s1, &s2);
            if dist_pairs.len() > 64 { // 12 beacons in common should cause this
                let new_beacons = correlate(&s1, &mut s2, dist_pairs);
                for b in new_beacons {
                    found_beacons.insert(b.clone());
                }
                known_scanners.push(s2.clone());
                continue 'unknown;
            }
        }

        // if we get here, s2 doesn't overlap with any known scanners...
        // put it at the front of the unknown list, opposite to the side we pop from
        let s2_clone = s2.clone();
        unknown_scanners.insert(0, s2.clone());
    }

    // Extra code for part 2:
    let mut largest_dist = 0;
    for s1 in known_scanners.iter() {
        for s2 in known_scanners.iter() {
            let dist = s1.location.manh_dist(&s2.location);
            if dist > largest_dist {
                largest_dist = dist;
            }
        }
    }

    println!("Answer for part two: {}", largest_dist);

    return Ok(found_beacons.len().to_string());
}

fn part_two(data: Vec<Scanner>) -> Result<String, Box<dyn Error>> {
    return Ok(String::from(""));
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn quat(&self) -> Quat {
        return Quat {
            a: 0.0,
            b: self.x as f64,
            c: self.y as f64,
            d: self.z as f64,
        }
    }

    fn manh_dist(&self, other: &Point) -> i32 {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();
        let z_dist = (self.z - other.z).abs();
        return x_dist + y_dist + z_dist;
    }
}

#[derive(Clone, Debug)]
struct Quat {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Quat {
    fn prod(&self, q: &Quat) -> Quat {
        return Self{
            a: self.a*q.a - self.b*q.b - self.c*q.c - self.d*q.d,
            b: self.a*q.b + self.b*q.a + self.c*q.d - self.d*q.c,
            c: self.a*q.c - self.b*q.d + self.c*q.a + self.d*q.b,
            d: self.a*q.d + self.b*q.c - self.c*q.b + self.d*q.a,
        }
    }

    fn rotate(&self, rot: &Quat) -> Quat {
        let rot_conj = Quat{a: rot.a, b: -rot.b, c: -rot.c, d: -rot.d};
        return rot.prod(self).prod(&rot_conj)
    }

    fn point(&self) -> Point {
        return Point {
            x: self.b.round() as i32,
            y: self.c.round() as i32,
            z: self.d.round() as i32,
        }
    }

    fn add(&self, other: &Quat) -> Quat {
        return Quat {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c,
            d: self.d + other.d,
        }
    }
}

impl PartialEq for Quat {
    fn eq(&self, other: &Self) -> bool {
        return self.a - EPSILON < other.a && other.a < self.a + EPSILON &&
            self.b - EPSILON < other.b && other.b < self.b + EPSILON &&
            self.c - EPSILON < other.c && other.c < self.c + EPSILON &&
            self.d - EPSILON < other.d && other.d < self.d + EPSILON
    }
}

fn get_rot_quats() -> Vec<Quat> {
    let mut v = Vec::new();
    let k = f64::sqrt(2.0)/2.0;

    // Identity
    v.push(Quat{a: 1.0, b: 0.0, c: 0.0, d: 0.0 });

    // rotations of 90° around each axis
    v.push(Quat{a: k,   b: k,   c: 0.0, d: 0.0 });
    v.push(Quat{a: 0.0, b: 1.0, c: 0.0, d: 0.0 });
    v.push(Quat{a: k,   b: -k,  c: 0.0, d: 0.0 });

    v.push(Quat{a: k,   b: 0.0, c: k,   d: 0.0 });
    v.push(Quat{a: 0.0, b: 0.0, c: 1.0, d: 0.0 });
    v.push(Quat{a: k,   b: 0.0, c: -k,  d: 0.0 });

    v.push(Quat{a: k,   b: 0.0, c: 0.0, d: k   });
    v.push(Quat{a: 0.0, b: 0.0, c: 0.0, d: 1.0 });
    v.push(Quat{a: k,   b: 0.0, c: 0.0, d: -k  });

    // rotations of 120° around vectors like <1,1,1>
    v.push(Quat{a: 0.5, b: 0.5, c: 0.5, d: 0.5 });
    v.push(Quat{a:-0.5, b: 0.5, c: 0.5, d: 0.5 });

    v.push(Quat{a: 0.5, b: 0.5, c: 0.5, d:-0.5 });
    v.push(Quat{a:-0.5, b: 0.5, c: 0.5, d:-0.5 });

    v.push(Quat{a: 0.5, b: 0.5, c:-0.5, d: 0.5 });
    v.push(Quat{a:-0.5, b: 0.5, c:-0.5, d: 0.5 });

    v.push(Quat{a: 0.5, b: 0.5, c:-0.5, d:-0.5 });
    v.push(Quat{a:-0.5, b: 0.5, c:-0.5, d:-0.5 });

    // rotations of 180° around vectors like <1,1,0>
    v.push(Quat{a: 0.0, b: k,   c: k,   d: 0.0 });
    v.push(Quat{a: 0.0, b: k,   c: -k,  d: 0.0 });

    v.push(Quat{a: 0.0, b: 0.0, c: k,   d: k   });
    v.push(Quat{a: 0.0, b: 0.0, c: k,   d: -k  });

    v.push(Quat{a: 0.0, b: k,   c: 0.0, d: k   });
    v.push(Quat{a: 0.0, b: k,   c: 0.0, d: -k  });

    return v;
}

#[derive(Clone, Debug)]
struct Scanner {
    beacons: Vec<Point>,
    distances: HashMap<[usize; 2], f64>,
    orientation: Quat,
    location: Point,
    location_known: bool,
}

const SCANNER_SELF_IDX: usize = 7777;

impl Scanner {
    fn new(points: Vec<Point>) -> Self {
        let len = points.len();
        let mut distances = HashMap::<[usize; 2], f64>::new();
        let mut i = 0;
        for p1 in points.iter() {
            for j in i+1..len {
                let p2 = points.get(j).unwrap();
                let x_sq = ((p1.x - p2.x) * (p1.x - p2.x)) as f64;
                let y_sq = ((p1.y - p2.y) * (p1.y - p2.y)) as f64;
                let z_sq = ((p1.z - p2.z) * (p1.z - p2.z)) as f64;
                let distance = f64::sqrt(x_sq + y_sq + z_sq);
                distances.insert([i, j], distance);
            }
            i += 1;
        }
        return Self{
            beacons: points, 
            distances: distances,
            orientation: Quat{a: 0.0, b: 0.0, c: 0.0, d: 0.0},
            location: Point{x: 0, y: 0, z: 0},
            location_known: false,
        };
    }

    fn get_dist(&self, i: usize, j: usize) -> Option<f64> {
        match self.distances.get(&[i,j]) {
            Some(d) => {
                return Some(*d);
            }
            None => {
                match self.distances.get(&[j,i]) {
                    Some(d) => {
                        return Some(*d);
                    }
                    None => {
                        return None;
                    }
                }
            }
        }
    }

    fn quat_vec(&self, i: usize, j:usize) -> Quat {
        let i_pt = if i != SCANNER_SELF_IDX {
            self.beacons.get(i).unwrap()
        } else {
            &Point{x: 0, y: 0, z: 0}
        };
        let j_pt = if j != SCANNER_SELF_IDX {
            self.beacons.get(j).unwrap()
        } else {
            &Point{x: 0, y: 0, z: 0}
        };
        return Quat{
            a: 0.0,
            b: (j_pt.x as f64) - (i_pt.x as f64),
            c: (j_pt.y as f64) - (i_pt.y as f64),
            d: (j_pt.z as f64) - (i_pt.z as f64),
        }
    }
}

fn matching_distances(s1: &Scanner, s2: &Scanner) -> Vec<[usize; 4]> {
    let mut v = Vec::new();
    for (pts1, d1) in s1.distances.iter() {
        for (pts2, d2) in s2.distances.iter() {
            // not sure if this "fudge" factor is necessary... floats are weird
            if d1 - EPSILON < *d2 && *d2 < d1 + EPSILON {
                v.push([pts1[0], pts1[1], pts2[0], pts2[1]])
            }
        }
    }
    return v;
}

fn correlate(s1: &Scanner, s2: &mut Scanner, dist_pairs: Vec<[usize; 4]>) -> Vec<Point> {
    let mut dp_iter = dist_pairs.iter();
    let pair1 = dp_iter.next().unwrap();
    let s1_idx1 = pair1[0];
    let s1_idx2 = pair1[1];

    // find another pair that uses s1_idx1, but not s1_idx2
    let pair2 = loop {
        let p = dp_iter.next().unwrap();
        if (p[0] == s1_idx1 && p[1] != s1_idx2) || (p[1] == s1_idx1 && p[0] != s1_idx2) {
            break p.clone()
        }
    };

    // use the second pair to identify which item from the s2 pair s1_idx1 corresponds to
    let (s2_idx1, s2_idx2) = if pair1[2] == pair2[2] || pair1[2] == pair2[3] {
        (pair1[2], pair1[3])
    } else {
        (pair1[3], pair1[2])
    };

    // make a (math) vector in each scanner's axes for the *same* pair and direction
    let q1 = s1.quat_vec(s1_idx1, s1_idx2);
    let q2 = s2.quat_vec(s2_idx1, s2_idx2);

    for rot in get_rot_quats() {
        let rotated = q2.rotate(&rot);
        if rotated == q1 {
            s2.orientation = s1.orientation.prod(&rot);
        }
    }
    if s2.orientation == (Quat{a: 0.0, b: 0.0, c: 0.0, d: 0.0}) {
        panic!("s2 orientation wasn't set");
    }

    let from_s1_to_idx1 = s1.quat_vec(SCANNER_SELF_IDX, s1_idx1).rotate(&s1.orientation);
    let from_idx1_to_s2 = s2.quat_vec(s2_idx1, SCANNER_SELF_IDX).rotate(&s2.orientation);
    s2.location = s1.location.quat().add(&from_s1_to_idx1).add(&from_idx1_to_s2).point();
    s2.location_known = true;
    
    let mut beacon_locs = Vec::new();

    for b in s1.beacons.iter() {
        let true_relative = b.quat().rotate(&s1.orientation);
        beacon_locs.push(true_relative.add(&s1.location.quat()).point());
    }

    for b in s2.beacons.iter() {
        let true_relative = b.quat().rotate(&s2.orientation);
        beacon_locs.push(true_relative.add(&s2.location.quat()).point());
    }

    return beacon_locs;
}

fn pt_ids(pts: &Vec<[usize; 4]>) -> (HashSet<usize>, HashSet<usize>) {
    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();

    for item in pts {
        s1.insert(item[0]);
        s1.insert(item[1]);
        s2.insert(item[2]);
        s2.insert(item[3]);
    }

    return (s1,s2);
}

fn read_data(path: String) -> Result<Vec<Scanner>, Box<dyn Error>> {
    let mut v = Vec::new();
    let mut first_gotten = false;
    let mut scanpoints = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        match record.len() {
            1 => {
                if first_gotten {
                    let scan = Scanner::new(scanpoints);
                    v.push(scan);
                    scanpoints = Vec::new();
                } else {
                    first_gotten = true;
                }
            },
            3 => {
                let mut iter = record.into_iter();
                let x = i32::from_str_radix(&iter.next().unwrap(), 10).unwrap();
                let y = i32::from_str_radix(&iter.next().unwrap(), 10).unwrap();
                let z = i32::from_str_radix(&iter.next().unwrap(), 10).unwrap();
                scanpoints.push(Point{x:x, y:y, z:z});
            },
            _ => {
                return Err("unexpected number of items in a line".into());
            }
        }
    }
    let scan = Scanner::new(scanpoints);
    v.push(scan);
    return Ok(v);
}
