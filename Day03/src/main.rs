use std::fs;

const SYMBOL_NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1), // northwest
    (-1,  0), // north
    (-1,  1), // northeast
    ( 0, -1), // west
    ( 0,  1), // east
    ( 1, -1), // southwest
    ( 1,  0), // south
    ( 1,  1), // southeast
];

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}


fn part1(contents: String) -> u64 {
    let mut ans: u64 = 0;
    let mut engine_layout: Vec<Vec<char>> = Vec::new();
    let mut symbol_locations: Vec<(u64, u64)> = Vec::new();

    for (row, line) in contents.lines().enumerate() {
        engine_layout.push(line.chars().collect());

        for (col, letter) in line.chars().enumerate() {
            if letter != '.' && (letter < '0' || letter > '9') {
                symbol_locations.push((row as u64, col as u64));
            }
        }
    }

    /* check around the symbols to see if there are numbers */
    let mut number_locs: Vec<(usize, usize)> = Vec::new();
    for (row, col) in symbol_locations {
        for (row_mod, col_mod) in SYMBOL_NEIGHBORS {
            let r: i64 = (row as i32 + row_mod) as i64;
            let c: i64 = (col as i32 + col_mod) as i64;

            if r >= 0 && c >= 0  && (r as usize) < engine_layout.len() && (c as usize) < engine_layout[0].len() {
                let l: char = engine_layout[r as usize][c as usize];
                if l >= '0' && l <= '9' {
                    /* check left and right of the location  to see if there are more numbers */
                    let mut ll: char = l;
                    let mut count: i64 = 1;
                    while ll >= '0' && ll <= '9' {
                        ll = if c - count >= 0 {
                            engine_layout[r as usize][(c - count) as usize]
                        } else {
                            '.'
                        };
                        count += 1;
                    }

                    /* count will always overcount by 1 (and it's initialized to 1), so undo both */
                    count -= 2;

                    let mut loop_flag: bool = true;
                    let mut num_as_str: String = String::new();
                    while loop_flag {
                        
                        
                        if !number_locs.contains(&(r as usize, (c - count) as usize)) {
                            let ch: char = if ((c-count) as usize) < engine_layout[0].len() {
                                 engine_layout[r as usize][(c - count) as usize]
                            } else { '.' };

                            if ch >= '0' && ch <= '9' {
                                num_as_str.push(ch);
                                number_locs.push((r as usize, (c - count) as usize));
                                count -= 1;
                            } else {
                                let num: u64 = num_as_str.parse::<u64>().unwrap();
                                ans += num;
                                loop_flag = false;
                            } 
                        } else {
                            loop_flag = false;
                        }
                    }
                }
            }
        }
    }

    return ans;
}


fn part2(contents: String) -> u64 {
    let mut ans: u64 = 0;
    let mut engine_layout: Vec<Vec<char>> = Vec::new();
    let mut symbol_locations: Vec<(u64, u64)> = Vec::new();

    for (row, line) in contents.lines().enumerate() {
        engine_layout.push(line.chars().collect());

        for (col, letter) in line.chars().enumerate() {
            if letter == '*' && (letter < '0' || letter > '9') {
                symbol_locations.push((row as u64, col as u64));
            }
        }
    }

    /* check around the symbols to see if there are numbers */
    let mut number_locs: Vec<(usize, usize)> = Vec::new();
    for (row, col) in symbol_locations {
        let mut gear_ratios: Vec<u64> = Vec::new();
        for (row_mod, col_mod) in SYMBOL_NEIGHBORS {
            let r: i64 = (row as i32 + row_mod) as i64;
            let c: i64 = (col as i32 + col_mod) as i64;

            if r >= 0 && c >= 0  && (r as usize) < engine_layout.len() && (c as usize) < engine_layout[0].len() {
                let l: char = engine_layout[r as usize][c as usize];
                if l >= '0' && l <= '9' {
                    /* check left and right of the location  to see if there are more numbers */
                    let mut ll: char = l;
                    let mut count: i64 = 1;
                    while ll >= '0' && ll <= '9' {
                        ll = if c - count >= 0 {
                            engine_layout[r as usize][(c - count) as usize]
                        } else {
                            '.'
                        };
                        count += 1;
                    }

                    /* count will always overcount by 1 (and it's initialized to 1), so undo both */
                    count -= 2;

                    let mut loop_flag: bool = true;
                    let mut num_as_str: String = String::new();
                    while loop_flag {
                        if !number_locs.contains(&(r as usize, (c - count) as usize)) {
                            let ch: char = if ((c-count) as usize) < engine_layout[0].len() {
                                 engine_layout[r as usize][(c - count) as usize]
                            } else { '.' };

                            if ch >= '0' && ch <= '9' {
                                num_as_str.push(ch);
                                number_locs.push((r as usize, (c - count) as usize));
                                count -= 1;
                            } else {
                                let num: u64 = num_as_str.parse::<u64>().unwrap();
                                gear_ratios.push(num);
                                loop_flag = false;
                            } 
                        } else {
                            loop_flag = false;
                        }
                    }
                }
            }
        }

        if gear_ratios.len() == 2 {
            ans += gear_ratios[0] * gear_ratios[1];
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
        assert_eq!(part1(contents.clone()), 4361);
    }

    #[test]
    fn test_part2() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 467835);
    }
}