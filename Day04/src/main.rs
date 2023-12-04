use std::fs;
use std::collections::{HashSet, HashMap};


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}

#[warn(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut ans: u64 = 0;

    for (_line_num, line) in contents.lines().enumerate() {
        let line_str: String = line.to_string();
        let temp: Vec<&str> = line_str.strip_prefix("Card ").unwrap().split(":").collect();
        let _card_num: u64 = temp[0].trim().parse().unwrap();

        let mut winning_nums: HashSet<u64> = HashSet::new();
        let mut current_nums: Vec<u64> = Vec::new();

        let mut cards: std::str::Split<'_, char> = temp[1].split('|');
        for winning_num in cards.next().unwrap().to_string().split_ascii_whitespace() {
            winning_nums.insert(winning_num.parse::<u64>().unwrap());
        }

        for current_num in cards.next().unwrap().to_string().split_ascii_whitespace() {
            current_nums.push(current_num.parse::<u64>().unwrap());
        }

        let mut match_count: u32 = 0;
        for current_num in current_nums {
            match_count += match winning_nums.contains(&current_num) {
                true => 1,
                _ => 0,
            };
        }

        if match_count > 0 {
            ans += (1 as u64) << (match_count - 1);
        }
    }

    return ans;
}


#[warn(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut ans: u64 = 0;
    let mut match_cards: HashMap<u64, u64> = HashMap::new();

    for (_line_num, line) in contents.lines().enumerate() {
        let line_str: String = line.to_string();
        let temp: Vec<&str> = line_str.strip_prefix("Card ").unwrap().split(":").collect();
        let card_num: u64 = temp[0].trim().parse().unwrap();

        let mut winning_nums: HashSet<u64> = HashSet::new();
        let mut current_nums: Vec<u64> = Vec::new();

        let mut cards: std::str::Split<'_, char> = temp[1].split('|');
        for winning_num in cards.next().unwrap().to_string().split_ascii_whitespace() {
            winning_nums.insert(winning_num.parse::<u64>().unwrap());
        }

        for current_num in cards.next().unwrap().to_string().split_ascii_whitespace() {
            current_nums.push(current_num.parse::<u64>().unwrap());
        }

        let mut match_count: u32 = 0;
        for current_num in current_nums {
            if winning_nums.contains(&current_num) {
                match_count += 1;
            }
        }

        /* TODO: count card copies */

        /* track the winning card copies in hashmap */
        let card_copies: u64 = *match_cards.get(&card_num).unwrap_or(&0);

        for _ in 0..card_copies+1 {
            if match_count > 0 {
                for i in card_num+1..card_num+1+(match_count as u64) {
                    if match_cards.contains_key(&card_num) {
                        let m: u64 = *match_cards.get(&card_num).unwrap();
                        match_cards.insert(i, m+1);
                        println!("updating: {i}:{}", m+1);
                    } else {
                        match_cards.insert(i, 1);
                        println!("creating: {i}:{}", 1);
                    }
                }
            }
        }
    }

    for (card_num, card_amount) in match_cards {
        ans += card_amount;

        println!("{}: {}", card_num, card_amount);
    }

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 13);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 30);
    }
}