#![allow(unused_variables)]

use std::error::Error;

fn main() {
    let ans1 = part_one(7, 10).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(7, 10).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

fn part_one(mut p1_pos: i32, mut p2_pos: i32) -> Result<String, Box<dyn Error>> {
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut next_roll = 1;
    let mut roll_count = 0;

    loop {
        if next_roll == 0 {
            next_roll = 100;
        }
        p1_pos = (p1_pos + next_roll) % 10;
        next_roll = (next_roll + 1) % 100;

        if next_roll == 0 {
            next_roll = 100;
        }
        p1_pos = (p1_pos + next_roll) % 10;
        next_roll = (next_roll + 1) % 100;

        if next_roll == 0 {
            next_roll = 100;
        }
        p1_pos = (p1_pos + next_roll) % 10;
        next_roll = (next_roll + 1) % 100;

        roll_count += 3;
        if p1_pos == 0 {
            p1_pos = 10;
        }
        p1_score += p1_pos;

        if p1_score >= 1000 {
            let ans = p2_score * roll_count;
            return Ok(ans.to_string());
        }

        if next_roll == 0 {
            next_roll = 100;
        }
        p2_pos = (p2_pos + next_roll) % 10;
        next_roll = (next_roll + 1) % 100;

        if next_roll == 0 {
            next_roll = 100;
        }
        p2_pos = (p2_pos + next_roll) % 10;
        next_roll = (next_roll + 1) % 100;

        if next_roll == 0 {
            next_roll = 100;
        }
        p2_pos = (p2_pos + next_roll) % 10;
        next_roll = (next_roll + 1) % 100;

        roll_count += 3;
        if p2_pos == 0 {
            p2_pos = 10;
        }
        p2_score += p2_pos;

        if p2_score >= 1000 {
            let ans = p1_score * roll_count;
            return Ok(ans.to_string());
        }
    }
}

fn part_two(p1_pos: i32, p2_pos: i32) -> Result<String, Box<dyn Error>> {
    let (p1_wins, p2_wins) = quantum_recurse(p1_pos, p2_pos, 0, 0);
    if p1_wins > p2_wins {
        return Ok(p1_wins.to_string());
    }
    return Ok(p2_wins.to_string());
}

fn quantum_recurse(p1_pos: i32, p2_pos: i32, p1_score: i32, p2_score: i32) -> (u64, u64) {
    let mut p1_wins = 0;
    let mut p2_wins = 0;

    let mut p1_change = 2;
    for n1 in vec![1,3,6,7,6,3,1] {
        p1_change += 1;
        let mut new_p1_pos = (p1_pos + p1_change) % 10;
        if new_p1_pos == 0 {
            new_p1_pos = 10;
        }

        let new_p1_score = p1_score + new_p1_pos;
        if new_p1_score >= 21 {
            p1_wins += n1;
            continue;
        }

        let mut p2_change = 2;
        for n2 in vec![1,3,6,7,6,3,1] {
            p2_change += 1;
            let mut new_p2_pos = (p2_pos + p2_change) % 10;
            if new_p2_pos == 0 {
                new_p2_pos = 10;
            }
    
            let new_p2_score = p2_score + new_p2_pos;
    
            if new_p2_score >= 21 {
                p2_wins += n1*n2;
                continue;
            }

            let (more_p1_wins, more_p2_wins) = quantum_recurse(new_p1_pos, new_p2_pos, new_p1_score, new_p2_score);
            p1_wins += more_p1_wins * n1 * n2;
            p2_wins += more_p2_wins * n1 * n2;
        }
    }

    return (p1_wins, p2_wins);
}
