use std::fs;

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}


fn part1(contents: String) -> u64 {
    /* 12 red, 13 green, 14 blue */

    let mut ans: u64 = 0;
    for line in contents.lines() {
        let line_str: String = line.to_string();
        let temp: Vec<&str> = line_str.strip_prefix("Game ").unwrap().split(":").collect();
        let game_num: u64 = temp[0].parse().unwrap();
        let mut valid_batch: bool = true;

        for batch in temp[1].split(";") {
            for color in batch.split(",") {
                let color_pair: Vec<&str> = color.split_ascii_whitespace().collect();
                let quantities: (u64, u64, u64) = match color_pair[1].to_uppercase().as_str() {
                    "RED" => (color_pair[0].parse().unwrap(), 0, 0),
                    "GREEN" => (0, color_pair[0].parse().unwrap(), 0),
                    "BLUE" => (0, 0, color_pair[0].parse().unwrap()),
                    _ => (0, 0, 0),
                };

                if quantities.0 > 12 ||
                    quantities.1 > 13 ||
                    quantities.2 > 14 {
                        valid_batch = false;
                        break;
                }
            }

            if !valid_batch {
                break;
            }
        }

        if valid_batch {
            ans += game_num;
        }
    }

    return ans;
}


fn part2(contents: String) -> u64 {
    let mut ans: u64 = 0;
    for line in contents.lines() {
        let line_str: String = line.to_string();
        let temp: Vec<&str> = line_str.strip_prefix("Game ").unwrap().split(":").collect();
        let mut red_max: u64 = 0;
        let mut blue_max: u64 = 0;
        let mut green_max: u64 = 0;

        for batch in temp[1].split(";") {
            for color in batch.split(",") {
                let color_pair: Vec<&str> = color.split_ascii_whitespace().collect();
                let quantities: (u64, u64, u64) = match color_pair[1].to_uppercase().as_str() {
                    "RED" => (color_pair[0].parse().unwrap(), 0, 0),
                    "GREEN" => (0, color_pair[0].parse().unwrap(), 0),
                    "BLUE" => (0, 0, color_pair[0].parse().unwrap()),
                    _ => (0, 0, 0),
                };

                if quantities.0 > red_max {
                    red_max = quantities.0;
                }
                if quantities.1 > green_max {
                    green_max = quantities.1;
                }
                if quantities.2 > blue_max {
                    blue_max = quantities.2;
                }
            }
        }

        ans += red_max*green_max*blue_max;
    }

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 8);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 2286);
    }
}