use std::error::Error;
// use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let ans1 = part_one().expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two().expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

fn part_one() -> Result<String, Box<dyn Error>> {
    let mut count = 0;

    for i in 123257..647015 {
        if is_valid(&i.to_string()) {
            count += 1;
        }
    }

    return Ok(count.to_string());
}

fn is_valid(s: &str) -> bool {
    if s.len() != 6 {
        return false;
    }

    let mut has_double = false;

    let mut prev = i32::MIN;
    for c in s.chars() {
        let val = i32::from_str_radix(&c.to_string(), 10).unwrap();
        if prev > val {
            return false;
        }

        if prev == val {
            has_double = true;
        }

        prev = val;
    }

    return has_double;
}

fn part_two() -> Result<String, Box<dyn Error>> {
    let mut count = 0;

    for i in 123257..647015 {
        if is_valid_p2(&i.to_string()) {
            count += 1;
        }
    }

    return Ok(count.to_string());
}

fn is_valid_p2(s: &str) -> bool {
    let mut counts = HashMap::<i32,i32>::new();

    if s.len() != 6 {
        return false;
    }

    let mut prev = i32::MIN;
    for c in s.chars() {
        let val = i32::from_str_radix(&c.to_string(), 10).unwrap();
        if prev > val {
            return false;
        }

        let this_count = counts.get_mut(&val);
        match this_count {
            Some(this_count) => {
                *this_count += 1;
            }
            None => {
                counts.insert(val, 1);
            }
        }

        prev = val;
    }

    for (_, v) in counts {
        if v == 2 {
            return true;
        }
    }

    return false;
}
