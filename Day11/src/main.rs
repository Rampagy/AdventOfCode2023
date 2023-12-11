mod position;

use std::fs;
use std::collections::HashSet;
use position::Position;
use std::cmp;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone(), 1000000));
}


#[inline(always)]
fn manhattan_distance(a: Position, b: Position) -> u64 {
    return (a.x - b.x).abs() as u64 + (a.y - b.y).abs() as u64;
}


#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut ans: u64 = 0;
    let mut galaxies: Vec<Position> = Vec::new();
    let mut map_height: i32 = 0;
    let mut map_width: i32 = 0;

    /* collect the galaxies */
    for (y, ch_row) in contents.lines().enumerate() {
        for (x, ch) in ch_row.chars().enumerate() {
            if ch == '#' {
                galaxies.push(Position::new(x as i32, y as i32));
            }
            map_width = x as i32;
        }
        map_height = y as i32;
    }

    map_width += 1;
    map_height += 1;

    /* loop through the galaxies to see which rows and cols are empty */
    let mut empty_rows: HashSet<i32> = HashSet::new();
    let mut empty_cols: HashSet<i32> = HashSet::new();
    for i in 0..map_width { empty_rows.insert(i); };
    for i in 0..map_height { empty_cols.insert(i); };

    for galaxy in galaxies.clone() {
        if empty_cols.contains(&galaxy.x) {
            empty_cols.remove(&galaxy.x);
        }
        if empty_rows.contains(&galaxy.y) {
            empty_rows.remove(&galaxy.y);
        }
    }

    /* figure out the distance from each galaxy to the others */
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let mut dist: i32 = manhattan_distance(galaxies[i], galaxies[j]) as i32;

            /* go through the empty cols/rows to see if any are in between the galaxies */
            let max_x: i32 = cmp::max(galaxies[i].x, galaxies[j].x);
            let min_x: i32 = cmp::min(galaxies[i].x, galaxies[j].x) + 1;
            for k in min_x..max_x {
                if empty_cols.contains(&k) {
                    dist += 1;
                }
            }

            let max_y: i32 = cmp::max(galaxies[i].y, galaxies[j].y);
            let min_y: i32 = cmp::min(galaxies[i].y, galaxies[j].y) + 1;
            for k in min_y..max_y {
                if empty_rows.contains(&k) {
                    dist += 1;
                }
            }

            ans += dist as u64;
        }
    }

    return ans;
}


#[allow(non_snake_case)]
fn part2(contents: String, expansion_amount: u64) -> u64 {
    let mut ans: u64 = 0;
    let mut galaxies: Vec<Position> = Vec::new();
    let mut map_height: i32 = 0;
    let mut map_width: i32 = 0;

    /* collect the galaxies */
    for (y, ch_row) in contents.lines().enumerate() {
        for (x, ch) in ch_row.chars().enumerate() {
            if ch == '#' {
                galaxies.push(Position::new(x as i32, y as i32));
            }
            map_width = x as i32;
        }
        map_height = y as i32;
    }

    map_width += 1;
    map_height += 1;

    /* loop through the galaxies to see which rows and cols are empty */
    let mut empty_rows: HashSet<i32> = HashSet::new();
    let mut empty_cols: HashSet<i32> = HashSet::new();
    for i in 0..map_width { empty_rows.insert(i); };
    for i in 0..map_height { empty_cols.insert(i); };

    for galaxy in galaxies.clone() {
        if empty_cols.contains(&galaxy.x) {
            empty_cols.remove(&galaxy.x);
        }
        if empty_rows.contains(&galaxy.y) {
            empty_rows.remove(&galaxy.y);
        }
    }

    /* figure out the distance from each galaxy to the others */
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let mut dist: u64 = manhattan_distance(galaxies[i], galaxies[j]);

            /* go through the empty cols/rows to see if any are in between the galaxies */
            let max_x: i32 = cmp::max(galaxies[i].x, galaxies[j].x);
            let min_x: i32 = cmp::min(galaxies[i].x, galaxies[j].x) + 1;
            for k in min_x..max_x {
                if empty_cols.contains(&k) {
                    dist += expansion_amount - 1;
                }
            }

            let max_y: i32 = cmp::max(galaxies[i].y, galaxies[j].y);
            let min_y: i32 = cmp::min(galaxies[i].y, galaxies[j].y) + 1;
            for k in min_y..max_y {
                if empty_rows.contains(&k) {
                    dist += expansion_amount - 1;
                }
            }

            ans += dist as u64;
        }
    }

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 374);
    }

    #[test]
    fn part2_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), 10), 1030);
        assert_eq!(part2(contents.clone(), 100), 8410);
    }
}