#![allow(unused_variables)]
#![allow(non_snake_case)]

use std::error::Error;
use std::collections::HashMap;

fn main() {
    let data = read_data(String::from("formatted.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(data.clone()).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

// big data: SNPVPFCPPKSBNSPSPSOF
// small data: NNCB
const STARTING_POLYMER: &str = "SNPVPFCPPKSBNSPSPSOF";

fn part_one(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let rules = get_basic_insertion_rules(data);

    let mut polymer = String::from(STARTING_POLYMER);
    for i in 0..10 {
        // up to 22 is do-able in a "reasonable" time with the small input
        // up to 19 with the real input.
        //println!("{}", i);
        polymer = expand_polymer(&rules, &polymer);
    }

    let cc = char_counts(&polymer);
    println!("{:#?}", cc);
    
    let ans = cc.big_small_diff();

    return Ok(ans.to_string());
}

// The insertion rules are a map from 2 chars to 3 chars - the polymer expander
 // will need to keep track of the extra character if it is just iterating over
 // adjacent elements in the polymer.
fn get_basic_insertion_rules(data: Vec<csv::StringRecord>) -> HashMap<String, String> {
    let mut rules = HashMap::new();

    for line in data {
        let mut line_iter = line.into_iter();
        let pair = line_iter.next().unwrap().to_string();
        let new_char = line_iter.next().unwrap().chars().next().unwrap();

        let mut result = pair.clone();
        result.insert(1, new_char);

        rules.insert(pair, result);

        match line_iter.next() {
            Some(v) => panic!("Unexpected additional item in iterator"),
            None => {},
        }
    }

    return rules;
}

fn expand_polymer(rules: &HashMap<String, String>, input: &String) -> String {
    let mut result = String::from("");

    let mut iter1 = input.chars();
    let mut iter2 = input.chars();
    iter2.next(); // offset the iterators

    for _ in 1..input.len() {
        let ch1 = iter1.next().unwrap();
        let ch2 = iter2.next().unwrap();
        
        let mut pair = String::from(ch1);
        pair.push(ch2);

        let mut ins = rules.get(&pair).unwrap().clone();
        ins.pop(); // remove last char from rule so we can append in place
        
        result.push_str(&ins);
    }

    let last_ch = iter1.next().unwrap();
    result.push(last_ch); // need to add the last char back in

    match iter1.next() {
        Some(v) => panic!("Unexpected additional item in iterator"),
        None => {},
    }

    return result;
}

#[derive(Debug,Clone)]
struct CharCounts {
    B: u64, C: u64, F: u64, H: u64, K: u64, N: u64, O: u64, P: u64, S: u64, V: u64,
}

impl CharCounts {
    fn inc(&mut self, c: char, n: u64) {
        match c {
            'B' => self.B += n,
            'C' => self.C += n,
            'F' => self.F += n,
            'H' => self.H += n,
            'K' => self.K += n,
            'N' => self.N += n,
            'O' => self.O += n,
            'P' => self.P += n,
            'S' => self.S += n,
            'V' => self.V += n,
            _ => panic!("Unexpected char (Not in [B,C,F,H,K,N,O,P,S,V]): {}", c),
        }
    }

    fn dec(&mut self, c: char, n: u64) {
        match c {
            'B' => self.B -= n,
            'C' => self.C -= n,
            'F' => self.F -= n,
            'H' => self.H -= n,
            'K' => self.K -= n,
            'N' => self.N -= n,
            'O' => self.O -= n,
            'P' => self.P -= n,
            'S' => self.S -= n,
            'V' => self.V -= n,
            _ => panic!("Unexpected char (Not in [B,C,F,H,K,N,O,P,S,V]): {}", c),
        }
    }

    fn add(&mut self, other: &CharCounts) {
        self.B += other.B;
        self.C += other.C;
        self.F += other.F;
        self.H += other.H;
        self.K += other.K;
        self.N += other.N;
        self.O += other.O;
        self.P += other.P;
        self.S += other.S;
        self.V += other.V;
    }

    fn big_small_diff(&self) -> u64 {
        let mut counts = Vec::new();
        // (no B)
        counts.push(self.C);
        counts.push(self.F);
        counts.push(self.H);
        counts.push(self.K);
        counts.push(self.N);
        counts.push(self.O);
        counts.push(self.P);
        counts.push(self.S);
        counts.push(self.V);

        let mut small = self.B;
        let mut big = self.B;
        for c in counts {
            if c == 0 {
                continue // the small data doesn't have all the characters
            }
            if c > big {
                big = c;
            }
            if c < small {
                small = c;
            }
        }

        return big - small;
    }
}

fn char_counts(input: &String) -> CharCounts {
    let mut counts = CharCounts{
        N: 0, S: 0, B: 0, V: 0, O: 0, C: 0, H: 0, K: 0, F: 0, P: 0,
    };

    for c in input.chars() {
        counts.inc(c, 1);
    }

    return counts;
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let rules_1 = get_basic_insertion_rules(data);
    let pairs = get_pairs(String::from(STARTING_POLYMER));

    let rules_5 = compound_rules(&rules_1, 5);
    let rules_10 = combine_rules(&rules_5, &rules_5);
    let rules_15 = combine_rules(&rules_5, &rules_10);

    let count_rules_25 = combine_to_counts(&rules_10, &rules_15);

    let count_rules_40 = combine_with_counts(&rules_15, pairs, &count_rules_25);
    
    println!("Expanding polymer...");
    let polymer = String::from(STARTING_POLYMER);
    let cc = expand_by_count(&count_rules_40, &polymer);
    println!("{:#?}", cc);
    
    let ans = cc.big_small_diff();

    return Ok(ans.to_string());
}

fn compound_rules(basic_rules: &HashMap<String, String>, factor: usize) -> HashMap<String, String> {
    let mut comp_rules = HashMap::new();

    let keys: Vec<String> = basic_rules.clone().into_keys().collect();

    let mut i = 1;
    let keynum = keys.len();

    for starting_pair in keys {
        println!("1/4 ... {} / {}", i, keynum);
        let mut result = starting_pair.clone();
        for i in 0..factor {
            result = expand_polymer(basic_rules, &result) 
        }
        comp_rules.insert(starting_pair.clone(), result);
        i += 1;
    }

    return comp_rules;
}

fn combine_rules(first_rules: &HashMap<String, String>, second_rules: &HashMap<String, String>) -> HashMap<String, String> {
    let mut combined = HashMap::new();

    let keys: Vec<String> = first_rules.clone().into_keys().collect();

    let mut i = 1;
    let keynum = keys.len();

    for starting_pair in keys {
        println!("2/4 ... {} / {}", i, keynum);
        let mut result = first_rules.get(&starting_pair).unwrap().clone();
        result = expand_polymer(second_rules, &result);
        combined.insert(starting_pair.clone(), result);
        i += 1;
    }

    return combined;
}

fn combine_to_counts(first_rules: &HashMap<String, String>, second_rules: &HashMap<String, String>) -> HashMap<String, CharCounts> {
    let mut countmap = HashMap::new();

    let keys: Vec<String> = first_rules.clone().into_keys().collect();

    let mut i = 1;
    let keynum = keys.len();

    for starting_pair in keys {
        println!("3/4 ... {} / {}", i, keynum);
        let result = first_rules.get(&starting_pair).unwrap();
        let count = expand_to_count(second_rules, result);
        countmap.insert(starting_pair.clone(), count);
        i += 1;
    }

    return countmap;
}

fn expand_to_count(rules: &HashMap<String, String>, input: &String) -> CharCounts {
    let mut result = char_counts(&String::from(""));

    let mut iter1 = input.chars();
    let mut iter2 = input.chars();
    iter2.next(); // offset the iterators

    for _ in 1..input.len() {
        let ch1 = iter1.next().unwrap();
        let ch2 = iter2.next().unwrap();

        let mut pair = String::from(ch1);
        pair.push(ch2);

        let mut ins = rules.get(&pair).unwrap().clone();
        ins.pop(); // remove last char from rule so we can append in place
        let count = char_counts(&ins);

        result.add(&count);
    }

    let last_ch = iter1.next().unwrap();
    // result.inc(last_ch, 1);
    
    match iter1.next() {
        Some(v) => panic!("Unexpected additional item in iterator"),
        None => {},
    }

    return result;
}

fn expand_by_count(count_rules: &HashMap<String, CharCounts>, input: &String) -> CharCounts {
    let mut result = char_counts(&String::from(""));

    let mut iter1 = input.chars();
    let mut iter2 = input.chars();
    iter2.next(); // offset the iterators

    for _ in 1..input.len() {
        let ch1 = iter1.next().unwrap();
        let ch2 = iter2.next().unwrap();

        let mut pair = String::from(ch1);
        pair.push(ch2);

        let count = count_rules.get(&pair).unwrap();

        result.add(count);
    }

    let last_ch = iter1.next().unwrap();
    result.inc(last_ch, 1);
    
    match iter1.next() {
        Some(v) => panic!("Unexpected additional item in iterator"),
        None => {},
    }

    return result;
}

fn combine_with_counts(rules: &HashMap<String, String>, keys: Vec<String>, count_rules: &HashMap<String, CharCounts>) -> HashMap<String, CharCounts> {
    let mut countmap = HashMap::new();

    let mut i = 1;
    let keynum = keys.len();
    
    for starting_pair in keys {
        println!("4/4 ... {} / {}", i, keynum);
        let result = rules.get(&starting_pair).unwrap();
        let mut count = expand_by_count(count_rules, result);
        let last_ch = starting_pair.chars().nth(1).unwrap();
        count.dec(last_ch, 1);
        countmap.insert(starting_pair.clone(), count);
        i += 1;
    }

    return countmap;
}

fn get_pairs(polymer: String) -> Vec<String> {
    let mut ans = Vec::new();

    let mut iter1 = polymer.chars();
    let mut iter2 = polymer.chars();
    iter2.next();

    for _ in 1..polymer.len() {
        let ch1 = iter1.next().unwrap();
        let ch2 = iter2.next().unwrap();

        let mut pair = String::from(ch1);
        pair.push(ch2);

        ans.push(pair.clone());
    }

    return ans
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
