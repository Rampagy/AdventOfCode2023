use std::fs;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}


#[warn(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut ans: u64 = 1;
    let mut times: Vec<u64> = vec![];
    let mut distances: Vec<u64> = vec![];
    let mut record_distances: Vec<u64> = vec![];
    let mut new_record_distances: Vec<u64> = vec![];

    for (line_num, line) in contents.lines().enumerate() {
        if line_num == 0 {
            times = line.to_string()
                        .split(":")
                        .nth(1).unwrap()
                        .split_ascii_whitespace()
                        .map(|x: &str| x.parse::<u64>().unwrap())
                        .collect();
        } else if line_num == 1 {
            record_distances = line.to_string()
                                .split(":")
                                .nth(1).unwrap()
                                .split_ascii_whitespace()
                                .map(|s| s.parse::<u64>().unwrap())
                                .collect();
        } else {
            /* no more data, shouldn't be getting here */
            break;
        }
    }

    for i in 0..times.len() {
        for hold_time in 0..times[i]+1 {
            let travel_time: u64 = times[i] - hold_time;

            /* hold_time doubles as the speed */
            distances.push(travel_time * hold_time);
        }

        for dist in distances.clone() {
            if dist > record_distances[i] {
                new_record_distances.push(dist);
            }
        }

        ans *= new_record_distances.len() as u64;
        distances.clear();
        new_record_distances.clear();
    }

    return ans;
}


#[warn(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut ans: u64 = 1;
    let mut times: Vec<String> = vec![];
    let mut distances: Vec<u64> = vec![];
    let mut record_distances: Vec<String> = vec![];
    let mut new_record_distances: Vec<u64> = vec![];

    for (line_num, line) in contents.lines().enumerate() {
        if line_num == 0 {
            times = line.to_string()
                        .split(":")
                        .nth(1).unwrap()
                        .split_ascii_whitespace()
                        .map(|s: &str| s.to_string())
                        .collect();
        } else if line_num == 1 {
            record_distances = line.to_string()
                                .split(":")
                                .nth(1).unwrap()
                                .split_ascii_whitespace()
                                .map(|s: &str| s.to_string())
                                .collect();
        } else {
            /* no more data, shouldn't be getting here */
            break;
        }
    }

    /* convert from list of string to concat */
    let record_dist: u64 = record_distances.concat().parse::<u64>().unwrap();
    let total_time: u64 = times.concat().parse::<u64>().unwrap();
    let mut time_from_start: u64 = 0;

    /* find the time required to beat the record distance */
    for hold_time in 0..total_time+1 {
        let travel_time: u64 = total_time - hold_time;
        let dist: u64 = travel_time * hold_time;

        if dist > record_dist {
            time_from_start = hold_time;
            break;
        }
    }

    ans = (total_time+1) - 2*time_from_start;

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 288);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 71503);
    }
}