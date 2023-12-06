#[allow(non_snake_case)]
use std::fs;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}

#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut ans: u64 = 0;
    let mut seeds: Vec<Vec<u64>> = Vec::new();
    let mut collecting_map: bool = false;
    let mut conversion_index: usize = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        if line.contains("seeds") {
            seeds.push(line.split(':')
                        .nth(1).unwrap()
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect());

            // initialize the other states to zero
            for _ in 0..7 as u64 {
                seeds.push(vec![u64::MAX; seeds[0].len()]);
            }

            continue;
        } else if line.contains("map") {
            collecting_map = true;

            /* continue to get to the lines that actually contain the numbers */
            continue; 
        } else if collecting_map && !line.is_empty() {
            /* generate the hashmap and then push it into the conversion maps vector */
            let nums: Vec<u64> = line.split_ascii_whitespace()
                                        .map(|s| s.parse::<u64>().unwrap())
                                        .collect();
            let start: u64 = nums[1];
            let goal: u64 = nums[0];
            let length: u64 = nums[2];

            for (i, current_conversion) in seeds[conversion_index].clone().iter().enumerate() {
                if start <= *current_conversion && *current_conversion < start + length {
                    seeds[conversion_index+1][i] = goal + *current_conversion - start;
                }
            }
        } else if collecting_map && line.is_empty() {
            /* reset the collecting_map flag and iterate conversion index */
            collecting_map = false;
            conversion_index += 1;

            /* go through and check to see if any conversions were missed and move the previous value up one */
            for (i, current_conversion) in seeds[conversion_index].clone().iter().enumerate() {
                if u64::MAX == *current_conversion {
                    seeds[conversion_index][i] = seeds[conversion_index-1][i];
                }
            }
        }
    }

    /* The file does not end in a new line so do the last layer translation here */
    let seed_length: usize = seeds.len();
    for (i, current_conversion) in seeds.last().unwrap().clone().iter().enumerate() {
        if u64::MAX == *current_conversion {
            seeds[seed_length-1][i] = seeds[seed_length-2][i];
        }
    }

    /* now iterate through the location layer and get the smallest */
    let mut min_val: u64 = u64::MAX;
    for loc in seeds.last().unwrap() {
        min_val = if *loc < min_val {
            *loc
        } else { 
            min_val
        };
    }

    ans = min_val;

    return ans;
}


#[allow(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut ans: u64 = 0;
    let mut seeds: Vec<u64> = Vec::new();
    let mut translation_layers: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut collecting_map: bool = false;
    let mut conversion_index: usize = 0;

    /* pre-populate the translation layers */
    for _ in 0..7 {
        translation_layers.push(Vec::new());
    }

    for (_line_num, line) in contents.lines().enumerate() {
        if line.contains("seeds") {
            seeds = line.split(':')
                        .nth(1).unwrap()
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect();
        } else if line.contains("map") {
            collecting_map = true;
        } else if collecting_map && !line.is_empty() {
            /* generate the hashmap and then push it into the conversion maps vector */
            let nums: Vec<u64> = line.split_ascii_whitespace()
                                        .map(|s| s.parse::<u64>().unwrap())
                                        .collect();
            /* start, goal, length */
            translation_layers[conversion_index].push((nums[1], nums[0], nums[2]));
        } else if collecting_map && line.is_empty() {
            /* reset the collecting_map flag and iterate conversion index */
            collecting_map = false;
            conversion_index += 1;
        }
    }

    /* iterate backwards through translation layers starting at starting at 0 location */
    translation_layers.reverse();

    for loc in 0..u64::MAX {
        let mut translation_value: u64 = loc;
        let mut ans_found: bool = false;

        for translation_layer in translation_layers.clone() {
            /* because we are traversing backwards through the layers the goal and start are opposite */
            for (goal, start, length) in translation_layer {
                if start <= translation_value && translation_value < start+length {
                    /* found a translation, get the new value and then break out of this layer */
                    translation_value = goal + translation_value - start;
                    break;
                }
            }
        }

        /* check to see if translation_value is a valid seed */
        for i in (0..seeds.len()).step_by(2) {
            if seeds[i] <= translation_value && translation_value < seeds[i] + seeds[i+1] {
                /* set answer and exit */
                ans = loc;
                ans_found = true;
                break;
            }
        }

        if ans_found {
            break;
        }
    }

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 35);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 46);
    }
}