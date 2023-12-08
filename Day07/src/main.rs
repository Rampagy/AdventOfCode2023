use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}


fn calculate_winner(mine: &[u8; 5], other: &[u8; 5]) -> Ordering {
    let mut mine_card_count: HashMap<u8, u64> = HashMap::new();
    let mut other_card_count: HashMap<u8, u64> = HashMap::new();

    /* count cards */
    for card in mine.clone() {
        if mine_card_count.contains_key(&card) {
            mine_card_count.insert(card, mine_card_count.get(&card).unwrap() + 1);
        } else {
            mine_card_count.insert(card, 1);
        }
    }

    for card in other.clone() {
        if other_card_count.contains_key(&card) {
            other_card_count.insert(card, other_card_count.get(&card).unwrap() + 1);
        } else {
            other_card_count.insert(card, 1);
        }
    }


    /* create a vector of card occurances and sort */
    let mut mine_occurances: Vec<u64> = mine_card_count.values().cloned().collect();
    let mut other_occurances: Vec<u64> = other_card_count.values().cloned().collect();

    mine_occurances.sort_by(|a, b| b.cmp(a));
    other_occurances.sort_by(|a, b| b.cmp(a));

    if mine_occurances[0] > other_occurances[0] {
        /* mine has more occurances of same card */
        return Ordering::Greater;
    } else if mine_occurances[0] < other_occurances[0] {
        /* other has more occurances of same card */
        return Ordering::Less;
    } else {
        /* same number of card occurances */
        if mine_occurances[0] == 3 && other_occurances[0] == 3 && 
                mine_occurances[1] == 2 && other_occurances[1] < 2 {
            /* mine is full house and other is not */
            return Ordering::Greater;
        } else if mine_occurances[0] == 3 && other_occurances[0] == 3 && 
                mine_occurances[1] < 2 && other_occurances[1] == 2 {
            /* other is full house and mine is not */
            return Ordering::Less;
        } else {
            if mine_occurances[0] == 2 && other_occurances[0] == 2 && 
                    mine_occurances[1] == 2 && other_occurances[1] < 2 {
                /* mine is two pair and other is not */
                return Ordering::Greater;
            } else if mine_occurances[0] == 2 && other_occurances[0] == 2 && 
                    mine_occurances[1] < 2 && other_occurances[1] == 2 {
                /* other is two pair and mine is not */
                return Ordering::Less;
            } else if mine_occurances[0] == 1 && other_occurances[0] == 1 {
                /* all distinct cards */
                if mine.iter().max() > other.iter().max() {
                    /* mine has high card */
                    return Ordering::Greater
                } else if mine.iter().max() < other.iter().max() {
                    /* other has high card */
                    return Ordering::Less
                } else { /* continue to tie breaker logic */ }
            }
        }
    }

    /* tie - go through each letter and until one of them is bigger */
    for i in 0..mine.len() {
        if mine[i] > other[i] {
            return Ordering::Greater;
        } else if mine[i] < other[i] {
            return Ordering::Less;
        } else { /* check next letter */ }
    }

    return Ordering::Equal;
}


#[warn(non_snake_case)]
fn part1(contents: String) -> u128 {
    let mut ans: u128 = 0;
    let mut cards: [[u8; 5]; 1000] = [[0; 5]; 1000];
    let mut card_bids: HashMap<[u8; 5], u16> = HashMap::new();

    let card_strength: HashMap<char, u8> = HashMap::from([
        ('A',  69),
        ('K',  68),
        ('Q',  67),
        ('J',  66),
        ('T',  58),
        ('9',  57),
        ('8',  56),
        ('7',  55),
        ('6',  54),
        ('5',  53),
        ('4',  52),
        ('3',  51),
        ('2',  50)
    ]);

    for (line_num, line) in contents.lines().enumerate() {
        for (i, val) in line.split_ascii_whitespace().enumerate() {
            if i == 0 {
                for (i, ch) in val.chars().enumerate() {
                    cards[line_num][i] = *card_strength.get(&ch).unwrap();
                }
            } else if i == 1 {
                if card_bids.insert(cards[line_num], val.parse::<u16>().unwrap()) != None {
                    println!("PANIC");
                }
            } else { /* shouldn't get here */ }
        }
    }

    cards.sort_by(calculate_winner);

    for i in 0..cards.len() {
        let card_val: u128 = *card_bids.get(&cards[i]).unwrap() as u128;
        ans += (i as u128+1)*card_val;
    }

    // 250927134
    return ans;
}


#[warn(non_snake_case)]
fn part2(contents: String) -> u64 {
    let mut ans: u64 = 1;


    return ans;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 6440);
    }

    #[test]
    fn test_calculate_winner() {
        /* Two pair */
        assert_eq!(calculate_winner(&[69, 69, 68, 68, 67], &[69, 68, 69, 68, 67]), Ordering::Greater);
        assert_eq!(calculate_winner(&[69, 68, 69, 68, 67], &[69, 69, 68, 68, 67]), Ordering::Less);

        /* full house */
        assert_eq!(calculate_winner(&[69, 69, 68, 68, 68], &[69, 68, 69, 68, 68]), Ordering::Greater);
        assert_eq!(calculate_winner(&[69, 68, 69, 68, 68], &[69, 69, 68, 68, 68]), Ordering::Less);

        /* 5 of a kind */
        assert_eq!(calculate_winner(&[10, 10, 10, 10, 10], &[6, 6, 6, 6, 10]), Ordering::Greater);
        assert_eq!(calculate_winner(&[6, 6, 6, 6, 10], &[10, 10, 10, 10, 10]), Ordering::Less);

        /* 4 of a kind */
        assert_eq!(calculate_winner(&[10, 6, 10, 10, 10], &[6, 6, 6, 6, 10]), Ordering::Greater);
        assert_eq!(calculate_winner(&[6, 6, 6, 6, 10], &[10, 6, 10, 10, 10]), Ordering::Less);

        /* 3 of a kind */
        assert_eq!(calculate_winner(&[10, 6, 10, 8, 10], &[6, 10, 6, 6, 8]), Ordering::Greater);
        assert_eq!(calculate_winner(&[6, 10, 6, 6, 8], &[10, 6, 10, 8, 10]), Ordering::Less);

        /* 2 of a kind */
        assert_eq!(calculate_winner(&[10, 9, 10, 7, 6], &[6, 7, 10, 9, 10]), Ordering::Greater);
        assert_eq!(calculate_winner(&[6, 7, 10, 9, 10], &[10, 9, 10, 7, 6]), Ordering::Less);

        /* 1 of a kind */
        assert_eq!(calculate_winner(&[10, 9, 8, 7, 6], &[6, 7, 8, 9, 10]), Ordering::Greater);
        assert_eq!(calculate_winner(&[6, 7, 8, 9, 10], &[10, 9, 8, 7, 6]), Ordering::Less);

        /* 4 of a kind & full house */
        assert_eq!(calculate_winner(&[10, 10, 10, 10, 6], &[10, 10, 10, 9, 9]), Ordering::Greater);
        assert_eq!(calculate_winner(&[10, 10, 10, 9, 9], &[10, 10, 10, 10, 6]), Ordering::Less);

        /* 3 of a kind & unique */
        assert_eq!(calculate_winner(&[10, 10, 10, 8, 9], &[10, 9, 8, 7, 6]), Ordering::Greater);
        assert_eq!(calculate_winner(&[10, 9, 8, 7, 6], &[10, 10, 10, 8, 9]), Ordering::Less);

        /* 5 of a kind & full house */
        assert_eq!(calculate_winner(&[10, 10, 10, 10, 10], &[9, 9, 8, 9, 8]), Ordering::Greater);
        assert_eq!(calculate_winner(&[9, 9, 8, 9, 8], &[10, 10, 10, 10, 10]), Ordering::Less);

        /* 5 of a kind & 5 of a kind */
        assert_eq!(calculate_winner(&[10, 10, 10, 10, 10], &[9, 9, 9, 9, 9]), Ordering::Greater);
        assert_eq!(calculate_winner(&[9, 9, 9, 9, 9], &[10, 10, 10, 10, 10]), Ordering::Less);

        /* 2 of a kind & 2 of a kind */
        assert_eq!(calculate_winner(&[7, 6, 7, 8, 12], &[5, 9, 3, 8, 8]), Ordering::Greater);
        assert_eq!(calculate_winner(&[5, 9, 3, 8, 8], &[7, 6, 7, 8, 12]), Ordering::Less);

        /* unique & unique */
        assert_eq!(calculate_winner(&[7, 6, 10, 8, 12], &[5, 9, 3, 4, 8]), Ordering::Greater);
        assert_eq!(calculate_winner(&[5, 9, 3, 4, 8], &[7, 6, 10, 8, 12]), Ordering::Less);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 71503);
    }
}