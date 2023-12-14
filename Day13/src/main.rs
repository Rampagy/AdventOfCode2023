use std::fs;
use std::collections::{HashSet, HashMap};
use std::cmp;


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone())); // 17743 - too low, 
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


fn get_reflection(graph: &str, solution: (bool, bool, usize)) -> (bool, bool, usize) {
    /* solution = use_solution, solution_is_horizontal, solution_reflection_index */
    let mut reflection_index: usize = 0;
    let mut reflection_was_horizontal: bool = true;
    let mut found_solution: bool = false;

    let reflections: HashSet<usize> = find_horizontal_reflections(graph);

    if reflections.len() == 1 && (!solution.0 || (solution.0 && (*reflections.iter().nth(0).unwrap() != solution.2 || !solution.1))) {
        reflection_index += reflections.iter().nth(0).unwrap();
        reflection_was_horizontal = true;
        found_solution = true;
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

        let reflections1: HashSet<usize> = find_horizontal_reflections(mirrored_graph.as_str());
        if reflections1.len() == 1 && (!solution.0 || (solution.0 && (*reflections1.iter().nth(0).unwrap() != solution.2 || solution.1))) {
            reflection_index += reflections1.iter().nth(0).unwrap();
            reflection_was_horizontal = false;
            found_solution = true;
        }
    }

    return (found_solution, reflection_was_horizontal, reflection_index);
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

    let mut graph_count: usize = 0;
    for graph in graphs {
        let original: (bool, bool, usize) = get_reflection(graph, (false, false, 0));

        let mut new_reflection: (bool, bool, usize) = (false, false, 0);
        let mut char_flip_idx: usize = 0;
        let graph_size: usize = graph.lines().count() * graph.lines().nth(0).unwrap().len();
        while char_flip_idx < graph_size {
            let row_of_flip: usize = char_flip_idx / graph.lines().nth(0).unwrap().len();
            let col_of_flip: usize = char_flip_idx % graph.lines().count();

            /* convert graph to vector of vectors so I can get rid of this abomination
               and go directly to the row/col I want to flip*/
            let mut modified_graph: String = "".to_string();
            for (row, line) in graph.lines().enumerate() {
                if row == row_of_flip {
                    let mut temp_line: String = "".to_string();
                    for (col, ch) in line.chars().enumerate() {
                        if col == col_of_flip {
                            /* flip this char */
                            if ch == '.' {
                                temp_line = format!("{}{}", temp_line, '#');
                            } else {
                                temp_line = format!("{}{}", temp_line, '.');
                            }
                        } else {
                            temp_line = format!("{}{}", temp_line, ch);
                        }
                    }
                    modified_graph = format!("{}{}", modified_graph, temp_line);
                } else {
                    modified_graph = format!("{}{}", modified_graph, line);
                }
                modified_graph = format!("{}\n", modified_graph);
            }

            new_reflection = get_reflection(modified_graph.as_str(), (true, original.1, original.2));

            if (original.1 != new_reflection.1 || original.2 != new_reflection.2) && new_reflection.0 {
               if new_reflection.1 {
                   /* was horizontal find */
                   ans += new_reflection.2;
               } else {
                   /* vertical find */
                   ans += 100*new_reflection.2;
               }
               
               break;
            }

            char_flip_idx += 1;
        }

        println!("{} {}", graph_count,  new_reflection.0);
        graph_count += 1;
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
        assert_eq!(part2(contents.clone()), 400);
    }
}