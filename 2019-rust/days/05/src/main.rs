use std::error::Error;
// use std::collections::HashSet;
// use std::collections::HashMap;

fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(&data).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(&data).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

const DEBUG: bool = false;

fn part_one(data: &Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let code = single_stream(data, 10)?;

    let ans = run_intcode(&code, &vec![1]);

    return Ok(format!("{:?}",ans));
}

fn part_two(data: &Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let code = single_stream(data, 10)?;

    let ans = run_intcode(&code, &vec![5]);

    return Ok(format!("{:?}",ans)); // 8641397 is too low
}

fn single_stream(data: &Vec<csv::StringRecord>, radix: u32) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut v = Vec::new();

    for line in data {
        for item in line.into_iter() {
            let n = i32::from_str_radix(item, radix)?;
            v.push(n);
            // for strings:
            // v.push(item.to_string());
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

fn run_intcode(code: &Vec<i32>, input: &Vec<i32>) -> Vec<i32> {
    let mut program = code.clone();
    let mut input = input.clone();
    input.reverse(); // this way, we can jut do pop
    let mut out = Vec::new();
    let mut pc = 0;

    loop {
        let instruction = program[pc];
        pc += 1;
        let opcode = instruction % 100;
        let mut parcode = instruction / 100;

        match opcode {
            1 => { // add 1 and 2, store at 3
                let pos1 = program[pc] as usize;
                pc += 1;
                let mode1 = parcode % 10;
                parcode /= 10;
                let val1 = match mode1 {
                    0 => program[pos1],
                    1 => pos1 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                let pos2 = program[pc] as usize;
                pc += 1;
                let mode2 = parcode % 10;
                // parcode /= 10;
                let val2 = match mode2 {
                    0 => program[pos2],
                    1 => pos2 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                let pos3 = program[pc] as usize;
                pc += 1;

                program[pos3] = val1 + val2;
                if DEBUG { 
                    println!("Instructions: {}, {}, {}, {}", instruction, pos1, pos2, pos3);
                    println!("Adding {} + {} and storing at {}", val1, val2, pos3);
                    println!("Program result: {:?}", program);
                    println!("pc = {}", pc);
                }
            },
            2 => { // multiply 1 and 2, store at 3
                let pos1 = program[pc] as usize;
                pc += 1;
                let mode1 = parcode % 10;
                parcode /= 10;
                let val1 = match mode1 {
                    0 => program[pos1],
                    1 => pos1 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                let pos2 = program[pc] as usize;
                pc += 1;
                let mode2 = parcode % 10;
                // parcode /= 10;
                let val2 = match mode2 {
                    0 => program[pos2],
                    1 => pos2 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                let pos3 = program[pc] as usize;
                pc += 1;

                program[pos3] = val1 * val2;
                if DEBUG { 
                    println!("Instructions: {}, {}, {}, {}", instruction, pos1, pos2, pos3);
                    println!("Multiplying {} + {} and storing at {}", val1, val2, pos3);
                    println!("Program result: {:?}", program);
                    println!("pc = {}", pc);
                }
            },
            3 => { // take an input, store at 1
                let pos1 = program[pc] as usize;
                let val = input.pop().unwrap();
                pc += 1;
                
                program[pos1] = val;
                if DEBUG { 
                    println!("Instructions: {}, {}", instruction, pos1);
                    println!("Got input {}, storing at {}", val, pos1);
                    println!("Program result: {:?}", program);
                    println!("pc = {}", pc);
                }
            },
            4 => { // output the value at 1
                let pos1 = program[pc] as usize;
                pc += 1;
                let mode1 = parcode % 10;
                // parcode /= 10;
                let val1 = match mode1 {
                    0 => program[pos1],
                    1 => pos1 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                out.push(val1);
                if DEBUG { 
                    println!("Instructions: {}, {}", instruction, pos1);
                    println!("Ouputting {}", val1);
                    println!("pc = {}", pc);
                }
            },
            5 => { // jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter.
                let pos1 = program[pc] as usize;
                pc += 1;
                let mode1 = parcode % 10;
                parcode /= 10;
                let val1 = match mode1 {
                    0 => program[pos1],
                    1 => pos1 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                let pos2 = program[pc] as usize;
                pc += 1;
                let mode2 = parcode % 10;
                // parcode /= 10;
                let val2 = match mode2 {
                    0 => program[pos2],
                    1 => pos2 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                if val1 != 0 {
                    pc = val2 as usize;
                }
                if DEBUG { 
                    println!("Instructions: {}, {}, {}", instruction, pos1, pos2);
                    println!("Jumping if {} is non-zero", val1);
                    println!("pc = {}", pc);
                }
            },
            6 => { // jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. 
                let pos1 = program[pc] as usize;
                pc += 1;
                let mode1 = parcode % 10;
                parcode /= 10;
                let val1 = match mode1 {
                    0 => program[pos1],
                    1 => pos1 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                let pos2 = program[pc] as usize;
                pc += 1;
                let mode2 = parcode % 10;
                // parcode /= 10;
                let val2 = match mode2 {
                    0 => program[pos2],
                    1 => pos2 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                if val1 == 0 {
                    pc = val2 as usize;
                }

                if DEBUG { 
                    println!("Instructions: {}, {}, {}", instruction, pos1, pos2);
                    println!("Jumping if {} is zero", val1);
                    println!("pc = {}", pc);
                }
            },
            7 => { // less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                let pos1 = program[pc] as usize;
                pc += 1;
                let mode1 = parcode % 10;
                parcode /= 10;
                let val1 = match mode1 {
                    0 => program[pos1],
                    1 => pos1 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                let pos2 = program[pc] as usize;
                pc += 1;
                let mode2 = parcode % 10;
                // parcode /= 10;
                let val2 = match mode2 {
                    0 => program[pos2],
                    1 => pos2 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                let pos3 = program[pc] as usize;
                pc += 1;

                if val1 < val2 {
                    program[pos3] = 1;
                } else {
                    program[pos3] = 0;
                }
                if DEBUG { 
                    println!("Instructions: {}, {}, {}, {}", instruction, pos1, pos2, pos3);
                    println!("storing result of {} < {} at {}", val1, val2, pos3);
                    println!("Program result: {:?}", program);
                    println!("pc = {}", pc);
                }
            },
            8 => { // equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                let pos1 = program[pc] as usize;
                pc += 1;
                let mode1 = parcode % 10;
                parcode /= 10;
                let val1 = match mode1 {
                    0 => program[pos1],
                    1 => pos1 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                let pos2 = program[pc] as usize;
                pc += 1;
                let mode2 = parcode % 10;
                // parcode /= 10;
                let val2 = match mode2 {
                    0 => program[pos2],
                    1 => pos2 as i32,
                    _ => panic!("Unknown parameter mode {}", mode1),
                };

                let pos3 = program[pc] as usize;
                pc += 1;

                if val1 == val2 {
                    program[pos3] = 1;
                } else {
                    program[pos3] = 0;
                }
                if DEBUG { 
                    println!("Instructions: {}, {}, {}, {}", instruction, pos1, pos2, pos3);
                    println!("storing result of {} == {} at {}", val1, val2, pos3);
                    println!("Program result: {:?}", program);
                    println!("pc = {}", pc);
                }
            },
            99 => {
                break;
            },
            other => {
                panic!("Unknown opcode {} at pc = {}", other, pc-1);
            }
        }
    }

    return out;
}
