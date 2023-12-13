use std::fs;
use std::collections::HashSet;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone())); // 6202 - too low, 7564 - too high
    println!("part 2: {}", part2(contents.clone()));
}


fn find_matches(spring_field: &str, num_groupings: usize, starting_idx: usize, 
                mut grouping_sizes: Vec<usize>, mut current_state: Vec<usize>, 
                mut found_states: HashSet<Vec<usize>>) -> HashSet<Vec<usize>> {

    let mut spring_field_count: usize = 0;
    let saved_indices: usize = grouping_sizes.iter().sum::<usize>() + grouping_sizes.len() - 1;
    while spring_field_count + saved_indices <= spring_field.len() {

        let mut valid_space_for_group: bool = true;
        for j in spring_field_count..spring_field_count+grouping_sizes[0] {
            /* search from the start to the end of the current grouping to see if it fits */
            if spring_field.chars().nth(j).unwrap_or('.') == '.' {
                valid_space_for_group = false;
                break;
            }
        }

        if valid_space_for_group {
            /* if there's valid space for a group, then check to make sure that 
             * it has appropriate spacing between the next potential grouping */
             if spring_field.chars().nth(spring_field_count+grouping_sizes[0]).unwrap_or('.') != '#' {
                /* valid group - do nothing */
                /* only a valid group if it is separated by a '.' or '?' */
                /* end of the line (or protion of the unwrap) is considered a valid boundary */
            } else {
                /* invalid group (insufficient spacing) - change the flag */
                valid_space_for_group = false;
            }
        }

        if valid_space_for_group {
            let grouping_end_location: usize = spring_field_count + grouping_sizes[0];

            /* save the starting index of the group as the state */
            let mut new_state: Vec<usize> = current_state.clone();
            let mut new_grouping_sizes: Vec<usize> = grouping_sizes.clone();
            new_state.push(spring_field_count+starting_idx);
            new_grouping_sizes.remove(0);

            if new_grouping_sizes.len() > 0 {
                /* get the new string slice */
                let new_spring_field: &str = &spring_field[(grouping_end_location + 1)..spring_field.len()];

                /* iterate past the # as they MUST be used */
                for i in starting_idx..(starting_idx+grouping_end_location+1) {
                    if spring_field.chars().nth(i-starting_idx).unwrap_or('.') == '#' {
                        if new_state[new_state.len()-1] > i /*|| i > starting_idx+grouping_end_location*/ {
                            /* there is an unused '#' in the previous characters */
                            /* that means this is an invalid branch - return found_states */
                            return found_states;
                        }
                    }
                }

                /* now search the new spring field */
                found_states = find_matches(new_spring_field, num_groupings, starting_idx+(grouping_end_location + 1), 
                                            new_grouping_sizes.clone(), new_state.clone(), found_states);
            } else {
                /* that was the last grouping - loop through to the end and make sure it's all dots */
                for i in grouping_end_location..spring_field.len() {
                    if spring_field.chars().nth(i).unwrap_or('.') == '#' {
                        // not clear to the end and we're out of groupings, invalid branch
                        /* invalid state- return found_states unchanged */
                        //return found_states;
                        /* TODO: this is the case that's broken */
                        spring_field_count += 1;
                        continue;
                    }
                }

                /* it's all clear to the end... add to found states and return it */
                /* do a quick check to make sure the current state length is right though first */
                if num_groupings == new_state.len()  && !found_states.contains(&new_state){
                    found_states.insert(new_state.clone());
                    //break;
                } else {
                    /* there are no groupings left to search, and somehow we don't have the correct amount in the state */
                    /* invalid state - return found_states unchanged */
                    //return found_states;
                }
            }
        }

        spring_field_count += 1;
    }

    return found_states;
}


#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut ans: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        /* get the grouping sizes */
        let grouping_sizes: Vec<usize> = line.split_ascii_whitespace().nth(1).unwrap()
                                            .split(',').map(|x| x.parse::<usize>().unwrap())
                                            .collect();

        let spring_field: &str = line.split_ascii_whitespace().nth(0).unwrap();

        /* set up a hashset to save the already found states */
        let mut found_states: HashSet<Vec<usize>> = HashSet::new();

        /* set up the initial current state (empty) */
        let current_state: Vec<usize> = Vec::new();

        /* find the matches */
        found_states = find_matches(spring_field, grouping_sizes.len(), 0, grouping_sizes, current_state, found_states);

        for s in found_states.clone() {
            for i in s {
                print!("{} ", i);
            }
            println!("");
        }

        println!("");
        ans += found_states.len() as u64;
    }

    return ans;
}


#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut ans: u64 = 0;

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 21);
    }

    #[test]
    fn part1_test2() {
        let grouping_sizes: Vec<usize> = vec![1, 1, 3];
        let mut found_states: HashSet<Vec<usize>> = HashSet::new();
        found_states = find_matches(".??..??...?##.", 3, 0, grouping_sizes, vec![], found_states);
        assert_eq!(found_states.len(), 4);
    }

    #[test]
    fn part1_test3() {
        let grouping_sizes: Vec<usize> = vec![1, 1, 7];
        let mut found_states: HashSet<Vec<usize>> = HashSet::new();
        found_states = find_matches(".???#??.?##?#???", 3, 0, grouping_sizes, vec![], found_states);
        assert_eq!(found_states.len(), 6);
    }

    #[test]
    fn part1_test4() {
        let grouping_sizes: Vec<usize> = vec![3, 2, 1];
        let mut found_states: HashSet<Vec<usize>> = HashSet::new();
        found_states = find_matches("?###????????", 3, 0, grouping_sizes, vec![], found_states);
        assert_eq!(found_states.len(), 10);
    }

    #[test]
    fn part1_test5() {
        let grouping_sizes: Vec<usize> = vec![2, 1, 1, 1];
        let mut found_states: HashSet<Vec<usize>> = HashSet::new();
        found_states = find_matches("?????#????#?", 4, 0, grouping_sizes, vec![], found_states);
        assert_eq!(found_states.len(), 10);
    }

    #[test]
    fn part2_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 1030);
        assert_eq!(part2(contents.clone()), 8410);
    }
}