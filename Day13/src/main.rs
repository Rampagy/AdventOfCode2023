use std::fs;
use std::collections::{HashSet, HashMap};
use std::cmp;


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn find_horizontal_reflections(graph: &str) -> HashSet<usize> {
    let mut valid_reflection_lines: HashSet<usize> = (1..graph.lines().nth(0).unwrap().len()).collect();

    for line in graph.lines() {
        for reflection_line in valid_reflection_lines.clone() {
            let mut valid_reflection_line: bool = true;
            for i in 0..cmp::min(reflection_line, line.len()-reflection_line) {
                if line.chars().nth(reflection_line-i-1).unwrap() != line.chars().nth(reflection_line+i).unwrap() {
                    valid_reflection_line = false;
                    break;
                }
            }

            if !valid_reflection_line {
                valid_reflection_lines.remove(&reflection_line);
            }
        }
    }

    return valid_reflection_lines;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part1(contents: String) -> usize {
    let mut ans: usize = 0;

    /* this split will only work in windows... */
    let graphs: Vec<&str> = contents.split("\r\n\r\n").collect();

    for graph in graphs {
        let reflections: HashSet<usize> = find_horizontal_reflections(graph);

        if reflections.len() == 1 {
            ans += reflections.iter().nth(0).unwrap();
        } else {
            /* need to search vertically */
            let mut mirrored_graph: String = "".to_string();
            let mut temp_conversion: Vec<Vec<char>> = Vec::new();
            for (line_num, line) in graph.lines().enumerate() {
                if line_num == 0 {
                    for j in 0..line.len() {
                        temp_conversion.push(vec![line.chars().nth(j).unwrap()]);
                    }
                } else {
                    for j in 0..line.len() {
                        temp_conversion[j].push(line.chars().nth(j).unwrap());
                    }
                }
            }

            for i in temp_conversion {
                for ch in i {
                    mirrored_graph = format!("{}{}", mirrored_graph, ch);
                }
                mirrored_graph = format!("{}\n", mirrored_graph);
            }

            ans += 100*find_horizontal_reflections(mirrored_graph.as_str()).iter().nth(0).unwrap();
        }
    }

    return ans;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part2(contents: String) -> usize {
    let mut ans: usize = 0;

    /* this split will only work in windows... */
    let graphs: Vec<&str> = contents.split("\r\n\r\n").collect();

    for graph in graphs {
        let mut original_reflection_horizontal: bool = true;
        let mut original_reflection: usize = 0;

        let reflections: HashSet<usize> = find_horizontal_reflections(graph);

        if reflections.len() == 1 {
            original_reflection += reflections.iter().nth(0).unwrap();
        } else {
            /* need to search vertically */
            let mut mirrored_graph: String = "".to_string();
            let mut temp_conversion: Vec<Vec<char>> = Vec::new();
            for (line_num, line) in graph.lines().enumerate() {
                if line_num == 0 {
                    for j in 0..line.len() {
                        temp_conversion.push(vec![line.chars().nth(j).unwrap()]);
                    }
                } else {
                    for j in 0..line.len() {
                        temp_conversion[j].push(line.chars().nth(j).unwrap());
                    }
                }
            }

            for i in temp_conversion {
                for ch in i {
                    mirrored_graph = format!("{}{}", mirrored_graph, ch);
                }
                mirrored_graph = format!("{}\n", mirrored_graph);
            }

            original_reflection += find_horizontal_reflections(mirrored_graph.as_str()).iter().nth(0).unwrap();
        }
    }

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)] #[allow(non_camel_case_types)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 405);
    }

    #[test]
    fn part2_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 8410);
    }
}