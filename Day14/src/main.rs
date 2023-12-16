use std::fs;
use std::collections::VecDeque;


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone())); // 17743 - too low, 18395 - too low,
}


fn mirror_over_xy(text: String) -> Vec<Vec<u8>> {
    /* Mirrors text over the line y = -x using new lines (\n or \r\n) as a delimiter.
        Returns the result as a Vec<Vec<u8>>.

    Example:

        input:
            xyz
            123

        output (but as their u8 equivalent):
            x1
            y2
            z3

        \
          \        x1
            \      y2
       xyz    \    z3
       123      \
                  \
     */

    let mut mirrored_text: Vec<Vec<u8>> = Vec::new();
    for (line_num, line) in text.lines().enumerate() {
        if line_num == 0 {
            for j in 0..line.len() {
                mirrored_text.push(vec![line.chars().nth(j).unwrap() as u8]);
            }
        } else {
            for j in 0..line.len() {
                mirrored_text[j].push(line.chars().nth(j).unwrap() as u8);
            }
        }
    }

    return mirrored_text;
}


fn convert_string_to_vec(text: String) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for (line_num, line) in text.lines().enumerate() {
        let mut temp: Vec<u8> = vec![];
        for ch in line.chars() {
            temp.push(ch as u8);
        }
        grid.push(temp);
    }

    return grid;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn rotate_90_clockwise(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut temp_grid: Vec<VecDeque<u8>> = vec![];

    for (i, line) in grid.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if i == 0 && j == 0 {
                for _ in 0..line.len() {
                    temp_grid.push(VecDeque::new());
                }
            }

            temp_grid[line.len()-j-1].push_front(*val);
        }
    }

    let mut new_grid: Vec<Vec<u8>> = vec![];
    for v in temp_grid {
        new_grid.push(v.into());
    }

    return new_grid;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn rotate_90_counterclockwise(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_grid: Vec<Vec<u8>> = vec![];

    for (i, line) in grid.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if i == 0 && j == 0 {
                for _ in 0..line.len() {
                    new_grid.push(vec![]);
                }
            }

            new_grid[line.len()-j-1].push(*val);
        }
    }

    return new_grid;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part1(contents: String) -> usize {
    let mut ans: usize = 0;
    let mut mirrored_contents: Vec<Vec<u8>> = mirror_over_xy(contents);

    /* this shifts the grid */
    for (i, line) in mirrored_contents.clone().iter().enumerate() {
        let mut rock_amount: usize = line.len();
        for (j, rock) in line.iter().enumerate() {
            if *rock == '#' as u8 {
                rock_amount = line.len() - j - 1;
            } else if *rock == 'O' as u8 {
                if rock_amount != line.len()-j {
                    mirrored_contents[i][line.len()-rock_amount] = 'O' as u8;
                    mirrored_contents[i][j] = '.' as u8;
                }
                rock_amount -= 1;
            }
        }
    }

    /* this scores the grid */
    for line in mirrored_contents {
        for (i, rock) in line.iter().enumerate() {
            if *rock == 'O' as u8 {
                ans += line.len()-i;
            }
        }
    }

    return ans;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part2(contents: String) -> usize {
    let mut ans: usize = 0;
    let mut grid: Vec<Vec<u8>> = convert_string_to_vec(contents);

    grid = rotate_90_counterclockwise(grid);

    for _ in 0..1000000000 {
        for _ in 0..4 { /* the 4 directions north, west, south, east */
            /* this shifts the round rocks left (west) */
            for (i, line) in grid.clone().iter().enumerate() {
                let mut rock_amount: usize = line.len();
                for (j, rock) in line.iter().enumerate() {
                    if *rock == '#' as u8 {
                        rock_amount = line.len() - j - 1;
                    } else if *rock == 'O' as u8 {
                        if rock_amount != line.len()-j {
                            grid[i][line.len()-rock_amount] = 'O' as u8;
                            grid[i][j] = '.' as u8;
                        }
                        rock_amount -= 1;
                    }
                }
            }

            grid = rotate_90_clockwise(grid);
        }
    }

    /* this scores the grid */
    for line in grid {
        for (i, rock) in line.iter().enumerate() {
            if *rock == 'O' as u8 {
                ans += line.len()-i;
            }
        }
    }

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)] #[allow(non_camel_case_types)]
mod tests {
    use super::*;

    #[test]
    fn mirror_over_xy_test() {
        assert_eq!(
            mirror_over_xy("xyz\n123".to_string()), 
            vec![vec![120, 49], vec![121, 50], vec![122, 51]]
        );
    }

    #[test]
    fn part1_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 136);
    }

    #[test]
    fn part2_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 64);
    }
}