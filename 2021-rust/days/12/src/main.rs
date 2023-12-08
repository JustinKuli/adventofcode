#![allow(unused_variables)]

use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let data = read_data(String::from("formatted.txt"))
        .expect("Failed to get data");

    let ans1 = part_one(data.clone()).expect("Failed to get answer 1");
    println!("Part one answer: {}", ans1);

    let ans2 = part_two(data.clone()).expect("Failed to get answer 2");
    println!("Part two answer: {}", ans2);
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
struct Cave {
    name: String,
    is_big: bool,
    adj: Vec<String>,
}

fn part_one(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let graph = build_graph(data);
    let ans = p1_paths_to_end(&graph, &vec![], "start".to_string());
    return Ok(ans.to_string());
}

fn p1_paths_to_end(graph: &HashMap::<String,Cave>, visited: &Vec::<String>, loc: String) -> u64 {
    if loc == "end".to_string() {
        return 1;
    }

    let curr = graph.get(&loc).unwrap();

    let mut new_visited = visited.clone();

    if !curr.is_big {
        new_visited.push(loc.clone());
    } 
    let mut sum = 0;
    for next in curr.clone().adj {
        if visited.contains(&next) {
            continue
        }
        sum += p1_paths_to_end(graph, &new_visited, next);
    }
    return sum;
}

fn build_graph(data: Vec<csv::StringRecord>) -> HashMap::<String,Cave> {
    let mut graph = HashMap::<String,Cave>::new();
    for line in data.clone() {
        let mut iter = line.into_iter();
        let c1 = iter.next().unwrap().to_string();
        let c2 = iter.next().unwrap().to_string();

        match graph.get(&c1) {
            Some(cave1) => {
                let mut new_cave1 = cave1.clone();
                if !new_cave1.adj.contains(&c2) {
                    new_cave1.adj.push(c2.clone());
                }
                graph.insert(c1.clone(), new_cave1);
            },
            None => {
                let mut hs = HashSet::new();
                hs.insert(c2.clone());
                graph.insert(c1.clone(), Cave{
                    name: c1.clone(),
                    is_big: c1.to_uppercase() == c1,
                    adj: vec![c2.clone()],
                });
            }
        }

        match graph.get(&c2) {
            Some(cave2) => {
                let mut new_cave2 = cave2.clone();
                if !new_cave2.adj.contains(&c1) {
                    new_cave2.adj.push(c1.clone());
                }
                graph.insert(c2.clone(), new_cave2);
            },
            None => {
                graph.insert(c2.clone(), Cave{
                    name: c2.clone(),
                    is_big: c2.to_uppercase() == c2,
                    adj: vec![c1],
                });
            }
        }
    }
    return graph;
}

fn part_two(data: Vec<csv::StringRecord>) -> Result<String, Box<dyn Error>> {
    let graph = build_graph(data);
    let ans = p2_paths_to_end(&graph, &vec![], "start".to_string());
    return Ok(ans.to_string());
}

fn p2_paths_to_end(graph: &HashMap::<String,Cave>, visited: &Vec::<String>, loc: String) -> u64 {
    if loc == "end".to_string() {
        return 1;
    }

    let curr = graph.get(&loc).unwrap();

    let mut new_visited = visited.clone();
    new_visited.push(loc.clone());
    if has_multiple_duplicate_small_caves(&new_visited) {
        return 0;
    }

    let mut sum = 0;
    for next in curr.clone().adj {
        if next == "start" {
            continue;
        }
        sum += p2_paths_to_end(graph, &new_visited, next);
    }
    return sum;
}

fn has_multiple_duplicate_small_caves(visited: &Vec::<String>) -> bool {
    let mut smalls = HashSet::<String>::new();
    let mut first_dupe_found = false;
    for c in visited {
        if c.clone() == c.to_uppercase() {
            continue
        }
        if smalls.contains(c) {
            if first_dupe_found {
                return true;
            } 
            first_dupe_found = true;
        }
        smalls.insert(c.clone());
    }
    return false;
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
