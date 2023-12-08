#![allow(unused_variables)]

use std::error::Error;

fn main() {
    let data = read_data(String::from("formatted.txt"))
        .expect("Failed to get data");
    
    let ans1 = part_one(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(data.clone()).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

fn get_numbers(data: Vec<csv::StringRecord>) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut v = Vec::new();

    let line1 = data.iter().next();
    if line1.is_none() {
        return Err("Could not get first line".into());
    }

    for item in line1.unwrap().into_iter() {
        let n = i32::from_str_radix(item, 10)?;
        v.push(n);
    }

    return Ok(v);
}

fn get_boards(data: Vec<csv::StringRecord>) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut v = Vec::new();

    let mut row_one = true;
    for line in data {
        if row_one { // skip the first row, it's the numbers to call out
            row_one = false;
            continue;
        }

        let mut board = Vec::new();
        for item in line.into_iter() {
            let n = i32::from_str_radix(item, 10)?;
            board.push(n);
        }
        v.push(board);
    }

    return Ok(v);
}

fn is_winner(board: Vec<i32>, drawn: Vec<i32>) -> bool {
    let multicheck = |c: Vec<usize>| {
        for n in c {
            if !drawn.contains(&board[n]) {
                return false;
            }
        }
        return true
    };

    // check rows
    if multicheck(vec![0,1,2,3,4]) {
        return true
    }
    if multicheck(vec![5,6,7,8,9]) {
        return true
    }
    if multicheck(vec![10,11,12,13,14]) {
        return true
    }
    if multicheck(vec![15,16,17,18,19]) {
        return true
    }
    if multicheck(vec![20,21,22,23,24]) {
        return true
    }

    // check columns
    if multicheck(vec![0,5,10,15,20]) {
        return true
    }
    if multicheck(vec![1,6,11,16,21]) {
        return true
    }
    if multicheck(vec![2,7,12,17,22]) {
        return true
    }
    if multicheck(vec![3,8,13,18,23]) {
        return true
    }
    if multicheck(vec![4,9,14,19,24]) {
        return true
    }

    return false;
}

fn score_board(board: Vec<i32>, drawn: Vec<i32>) -> i32 {
    let mut unused_board = board.clone();
    for d in drawn.clone() {
        let mut i = 0;
        while i < unused_board.len() {
            while i < unused_board.len() && unused_board[i] == d {
                unused_board.swap_remove(i);
            }
            i+=1;
        }
    }

    let mut sum = 0;
    for val in unused_board {
        sum += val;
    }
    let cumsum = sum * drawn[drawn.len()-1];
    return cumsum;
}

fn part_one(data: Vec<csv::StringRecord>) -> Result<i32, Box<dyn Error>> {
    let numbers = get_numbers(data.clone())?;
    let boards = get_boards(data.clone())?;

    let mut drawn = Vec::new();
    for n in numbers {
        drawn.push(n);

        for b in boards.clone() {
            if is_winner(b.clone(), drawn.clone()) {
                return Ok(score_board(b.clone(), drawn.clone()));
            }
        }
    }

    return Err("no winning board found".into());
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<i32, Box<dyn Error>> {
    let numbers = get_numbers(data.clone())?;
    let mut boards = get_boards(data.clone())?;

    let mut drawn = Vec::new();
    for n in numbers {
        drawn.push(n);

        let mut i = 0;
        while i < boards.len() {
            while i < boards.len() && is_winner(boards[i].clone(), drawn.clone()) {
                if boards.len() == 1 {
                    return Ok(score_board(boards[0].clone(), drawn.clone()));
                }
               boards.remove(i);
            }
            i+=1;
        }
    }

    return Err("no last winning board found".into());
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
