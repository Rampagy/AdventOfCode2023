use std::fs;
use std::collections::{HashSet, HashMap};
use std::str::Chars;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}


#[warn(non_snake_case)]
fn part1(contents: String) -> u128 {
    let mut ans: u128 = 0;
    let mut lines: std::str::Lines<'_>  = contents.lines();
    let directions: &str = lines.nth(0).unwrap();
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    let mut current_node: String = "AAA".to_string();

    /* skip the first one as that is the directions */
    for (i, line) in lines.enumerate() {
        if i > 0 {
            let source: &str = line.split(" = ").nth(0).unwrap();
            let dests: String = line.split(" = ").nth(1).unwrap().replace("(", "").as_str().replace(")", "");
            let destinations: (&str, &str) = (dests.split(", ").nth(0).unwrap(), dests.split(", ").nth(1).unwrap());

            graph.insert(source.to_string(), (destinations.0.to_string(), destinations.1.to_string()));
        }
    }

    let mut loop_count: u128 = 1;
    let mut dir_idx: usize = 0;
    loop {
        let a: &(String, String) = graph.get(&current_node).unwrap();
        let ldest: String = a.0.clone();
        let rdest: String = a.1.clone();

        let dir: char = directions.chars().nth(dir_idx).unwrap();
        if dir == 'L' {
            current_node = ldest;
        } else if  dir == 'R' {
            current_node = rdest;
        }

        if current_node == "ZZZ" {
            ans = loop_count;
            break;
        } else {
            loop_count += 1;
            dir_idx = (dir_idx + 1) % directions.len();
        }
    }

    return ans;
}


#[warn(non_snake_case)]
fn part2(contents: String) -> u128 {
    let mut ans: u128 = 0;
    let mut lines: std::str::Lines<'_>  = contents.lines();
    let directions: &str = lines.nth(0).unwrap();
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    let mut current_nodes: Vec<String> = Vec::new();
    let mut end_nodes: HashSet<String> = HashSet::new();

    /* skip the first one as that is the directions */
    for (i, line) in lines.enumerate() {
        if i > 0 {
            let source: &str = line.split(" = ").nth(0).unwrap();
            let dests: String = line.split(" = ").nth(1).unwrap().replace("(", "").as_str().replace(")", "");
            let destinations: (&str, &str) = (dests.split(", ").nth(0).unwrap(), dests.split(", ").nth(1).unwrap());

            graph.insert(source.to_string(), (destinations.0.to_string(), destinations.1.to_string()));

            if source.ends_with("A") {
                current_nodes.push(source.to_string());
            }

            if source.ends_with("Z") {
                end_nodes.insert(source.to_string());
            }
        }
    }

    let mut loop_count: u128 = 1;
    let mut dir_idx: usize = 0;
    loop {
        let dir: char = directions.chars().nth(dir_idx).unwrap();
        let mut temp_current_node: Vec<String> = Vec::new();
        let mut winner: bool = true;

        for current_node in &current_nodes {
            let x: Chars<'_> = current_node.chars();
            let a: &(String, String) = graph.get(&x.as_str().to_string()).unwrap();
            let ldest: String = a.0.clone();
            let rdest: String = a.1.clone();

            let mut new_node: String = "".to_string();
            if dir == 'L' {
                new_node = ldest;
            } else if  dir == 'R' {
                new_node = rdest;
            }

            temp_current_node.push(new_node);
        }

        current_nodes = temp_current_node;

        
        for current_node in &current_nodes {
            if !end_nodes.contains(current_node) {
                winner = false;
                break;
            }
        }

        if winner {
            ans = loop_count;
            break;
        } else {
            loop_count += 1;
            dir_idx = (dir_idx + 1) % directions.len();
        }
    }

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 2);
    }

    #[test]
    fn test2_part1() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 6);
    }

    #[test]
    fn test3_part2() {
        let contents: String = fs::read_to_string("src/test3.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 6);
    }
}