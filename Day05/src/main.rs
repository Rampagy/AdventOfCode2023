use std::fs;
use std::time::Instant;
use std::thread;

const NTHREADS: u64 = 64;
const BATCH_SIZE: u64 = 4000000;

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut start_time: Instant = Instant::now();
    let mut ans: u64 =  part1(contents.clone());
    let mut duration: std::time::Duration = start_time.elapsed();
    println!("part 1: {} ({} sec)", ans, duration.as_secs() as f64 + 1e-9 as f64*duration.subsec_nanos() as f64);

    start_time = Instant::now();
    ans = part2_single(contents.clone());
    duration = start_time.elapsed();
    println!("part 2 (1 thread): {} ({} sec)", ans, duration.as_secs() as f64 + 1e-9 as f64*duration.subsec_nanos() as f64);

    start_time = Instant::now();
    ans = part2_multi(contents.clone());
    duration = start_time.elapsed();
    println!("part 2 ({} threads): {} ({} sec)", NTHREADS, ans, duration.as_secs() as f64 + 1e-9 as f64*duration.subsec_nanos() as f64);
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
fn part2_single(contents: String) -> u64 {
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


#[allow(non_snake_case)]
fn part2_multi(contents: String) -> u64 {
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

    for loc in (0..u64::MAX).step_by((BATCH_SIZE*NTHREADS) as usize) {
        let mut children: Vec<thread::JoinHandle<(bool, u64)>> = vec![];
        for thd in 0..NTHREADS {
            let tl_copy: Vec<Vec<(u64, u64, u64)>> = translation_layers.clone();
            let seeds_copy: Vec<u64> = seeds.clone();
            let loc_start: u64 = loc+thd*BATCH_SIZE;
            let loc_end: u64 = loc+(thd+1)*BATCH_SIZE;

            children.push(thread::spawn(move || {
                thread::sleep(std::time::Duration::from_millis(500));
                is_valid_seed(loc_start, loc_end, tl_copy, seeds_copy, thd)
            }));
        }

        let mut results: Vec<u64> = vec![];
        for child in children {
            let ans_found: bool;
            let loc: u64;

            /* collect results */
            (ans_found, loc) = child.join().unwrap();

            if ans_found {
                results.push(loc);
            }
        }

        if !results.is_empty() {
            ans = *results.iter().min().unwrap();
            break;
        }
    }

    return ans;
}


#[allow(non_snake_case)]
fn is_valid_seed(loc_start: u64, loc_end: u64, translation_layers: Vec<Vec<(u64, u64, u64)>>, seeds: Vec<u64>, _thd: u64) -> (bool, u64) {
    let mut answers: Vec<u64> = vec![];
    for l in loc_start..loc_end {
        let mut translation_value: u64 = l;

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
        for i in (0..seeds.len() as usize).step_by(2) {
            if seeds[i] <= translation_value && translation_value < seeds[i] + seeds[i+1] {
                /* set answer and exit */
                answers.push(l);
                break;
            }
        }
    }

    if !answers.is_empty() {
        return (true, *answers.iter().min().unwrap())
    }

    return (false, 0)
}


#[allow(non_snake_case)]
fn thread_experiment() -> Vec<char> {
    let char_vec: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let mut children: Vec<thread::JoinHandle<char>> = vec![];
    let mut result_vec: Vec<char> = vec![];

    for ch in char_vec {
        children.push(thread::spawn(move || {
            capitalize(ch)
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        result_vec.push(child.join().unwrap());
    }

    return result_vec;
}


#[allow(non_snake_case)]
fn capitalize(a: char) -> char {
    return a.to_ascii_uppercase();
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
    fn test_part2_single() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2_single(contents.clone()), 46);
    }

    #[test]
    fn test_part2_multi() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2_multi(contents.clone()), 46);
    }

    #[test]
    fn test_thread_experiment() {
        assert_eq!(thread_experiment(), vec!['A', 'B', 'C', 'D', 'E']);
    }
}