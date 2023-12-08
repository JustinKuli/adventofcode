use std::error::Error;
// use std::collections::HashSet;
// use std::collections::HashMap;

fn main() {
    let data = read_data(String::from("data.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(&data).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1); // 797908 is too low?

    let ans2 = part_two(&data).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

const DEBUG: bool = false;

fn part_one(data: &Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let mut items = single_stream(data, 10)?;

    let mem1 = items.get_mut(1).unwrap();
    *mem1 = 12;
    let mem2 = items.get_mut(2).unwrap();
    *mem2 = 2;

    let final_state = run_program(&items);
    
    return Ok(final_state.get(0).unwrap().to_string());
}

fn run_program(data: &Vec<i32>) -> Vec<i32> {
    let mut program = data.clone();
    let mut pc: usize = 0;

    loop {
        if DEBUG { println!("program state: {:?}", program)}
        if let Some(opcode) = program.get(pc) {
            match *opcode {
                1 => { // add
                    let pos1 = *program.get(pc+1).unwrap() as usize;
                    let val1 = *program.get(pos1).unwrap();
                    let pos2 = *program.get(pc+2).unwrap() as usize;
                    let val2 = *program.get(pos2).unwrap();
                    let pos3 = *program.get(pc+3).unwrap() as usize;
                    if let Some(dest) = program.get_mut(pos3) {
                        if DEBUG { println!("Adding {}={} and {}={} to pos {}", pos1, val1, pos2, val2, pos3)}
                        *dest = val1 + val2;
                    } else {
                        panic!("Couldn't get ref to item {} from program", pos3);
                    }
                },
                2 => { // multiply
                    let pos1 = *program.get(pc+1).unwrap() as usize;
                    let val1 = *program.get(pos1).unwrap();
                    let pos2 = *program.get(pc+2).unwrap() as usize;
                    let val2 = *program.get(pos2).unwrap();
                    let pos3 = *program.get(pc+3).unwrap() as usize;
                    if let Some(dest) = program.get_mut(pos3) {
                        if DEBUG { println!("Multiplying {}={} and {}={} to pos {}", pos1, val1, pos2, val2, pos3)}
                        *dest = val1 * val2;
                    } else {
                        panic!("Couldn't get ref to item {} from program", pos3);
                    }
                },
                99 => {
                    break;
                },
                _ => {
                    panic!("Unknown opcode {} at position {}", opcode, pc);
                }
            }
        } else {
            panic!("Couldn't lookup opcode at {} from program", pc);
        }
        pc += 4;
    }

    return program;
}

fn part_two(data: &Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let mut items = single_stream(data, 10)?;

    for noun in 0..100 {
        for verb in 0..100 {
            let mem1 = items.get_mut(1).unwrap();
            *mem1 = noun;
            let mem2 = items.get_mut(2).unwrap();
            *mem2 = verb;

            let output = run_program(&items);
            if *output.get(0).unwrap() == 19690720 {
                let ans = 100 * noun + verb;
                return Ok(ans.to_string());
            }
        }
    }
    
    return Err("No correct input found".into());
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
