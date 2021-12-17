#![allow(unused_variables)]

use std::error::Error;

fn main() {
    let ans1 = part_one(169, 206, -108, -68).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(169, 206, -108, -68).expect("Failed to get answer 2");
    //let ans2 = part_two(20, 30, -10, -5).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

fn part_one(x1: i32, x2: i32, y1: i32, y2: i32) -> Result<String, Box<dyn Error>> {
    let best_y = -1*(y1+1);
    let mut sum = best_y;
    for i in 1..best_y {
        sum += i;
    }
    return Ok(sum.to_string());
}

fn part_two(x1: i32, x2: i32, y1: i32, y2: i32) -> Result<String, Box<dyn Error>> {
    let best_y = -1*(y1+1);
    
    // these ranges are probably bigger than necessary, but w/e
    let mut acc = 0;
    for y_v in -(best_y+1)..(best_y+1) {
        for x_v in 0..(x2+1) {
            if hits_target(x_v, y_v, x1, x2, y1, y2) {
                acc += 1;
            }
        }
    }

    return Ok(acc.to_string());
}

fn hits_target(x_vel: i32, y_vel: i32, x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut x_v = x_vel;
    let mut y_v = y_vel;

    while y >= y1 && x <= x2 {
        if x >= x1 && y <= y2 {
            return true;
        }

        x += x_v;
        y += y_v;
        if x_v != 0 {
            x_v -= 1;
        }
        y_v -= 1;
    }

    return false;
}
