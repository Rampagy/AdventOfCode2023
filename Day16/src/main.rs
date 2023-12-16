use std::fs;
use std::collections::HashSet;

#[allow(non_camel_case_types)]
enum Orientation {
    North,
    East,
    South,
    West,
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}

#[allow(non_snake_case)]
fn convert_string_to_vec(text: String) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in text.lines() {
        let mut temp: Vec<u8> = vec![];
        for ch in line.chars() {
            temp.push(ch as u8);
        }
        grid.push(temp);
    }

    return grid;
}

#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn get_next_square_from_orientation(r: usize, c: usize, dir: u8) -> Vec<(usize, usize)> {
    let mut new_locs: Vec<(usize, usize)> = vec![];

    if r == 0  && dir == Orientation::North as u8 {
        return new_locs;
    } else if c == 0 && dir == Orientation::West as u8 {
        return new_locs;
    }

    if dir == Orientation::North as u8 {
        new_locs.push((r-1, c));
    } else if dir == Orientation::East as u8 {
        new_locs.push((r, c+1));
    } else if dir == Orientation::South as u8 {
        new_locs.push((r+1, c));
    } else if dir == Orientation::West as u8 {
        new_locs.push((r, c-1));
    }

    return new_locs;
}

fn get_next_location(grid: Vec<Vec<u8>>, r: usize, c: usize, going_dir: u8) -> Vec<(usize, usize, u8)> {
    let mut new_dir: u8 = Orientation::North as u8;
    let mut new_r: usize = 0;
    let mut new_c: usize = 0;
    let mut ret_locs: Vec<(usize, usize, u8)> = vec![];

    if going_dir == Orientation::East as u8 {
        if grid[r][c] == '\\' as u8 {
            /* goes south */
            new_dir = Orientation::South as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else if grid[r][c] == '/' as u8 {
            /* goes north */
            new_dir = Orientation::North as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else if grid[r][c] == '|' as u8 {
            /* splits and goes south/north */
            new_dir = Orientation::North as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));

            new_dir = Orientation::South as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else {
            /* continues in current direction (east) */
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, going_dir);
            if a .len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, going_dir));
        }
    } else if going_dir == Orientation::West as u8 {
        if grid[r][c] == '/' as u8 {
            /* goes south */
            new_dir = Orientation::South as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else if grid[r][c] == '\\' as u8 {
            /* goes north */
            new_dir = Orientation::North as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else if grid[r][c] == '|' as u8 {
            /* splits and goes south/north */
            new_dir = Orientation::North as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));

            new_dir = Orientation::South as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else {
            /* continues in current direction (east) */
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, going_dir);
            if a .len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, going_dir));
        }
    } else if going_dir == Orientation::North as u8 {
        if grid[r][c] == '\\' as u8 {
            /* goes west */
            new_dir = Orientation::West as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else if grid[r][c] == '/' as u8 {
            /* goes east */
            new_dir = Orientation::East as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else if grid[r][c] == '-' as u8 {
            /* splits and goes west/east */
            new_dir = Orientation::West as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));

            new_dir = Orientation::East as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else {
            /* continues in current direction (north) */
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, going_dir);
            if a .len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, going_dir));
        }
    } else if going_dir == Orientation::South as u8 {
        if grid[r][c] == '\\' as u8 {
            /* goes east */
            new_dir = Orientation::East as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else if grid[r][c] == '/' as u8 {
            /* goes west */
            new_dir = Orientation::West as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else if grid[r][c] == '-' as u8 {
            /* splits and goes west/east */
            new_dir = Orientation::West as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));

            new_dir = Orientation::East as u8;
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, new_dir);
            if a.len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, new_dir));
        } else {
            /* continues in current direction (south) */
            let a: Vec<(usize, usize)> = get_next_square_from_orientation(r, c, going_dir);
            if a .len() > 0 {
                (new_r, new_c) = a[0];
            }
            ret_locs.push((new_r, new_c, going_dir));
        }
    }

    return ret_locs;
}

fn get_border_vals_and_dir(grid: Vec<Vec<u8>>, grid_width: usize, grid_height: usize) -> Vec<(usize, usize, u8)> {
    let mut border_coords: Vec<(usize, usize, u8)> = vec![];

    /* TODO */
    

    return border_coords;
}

#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part1(contents: String) -> usize {
    let mut ans: usize = 0;
    let grid: Vec<Vec<u8>> = convert_string_to_vec(contents);
    let mut open_locations: Vec<(usize, usize, u8)> = vec![];
    let mut visited_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut laser_cache: HashSet<(usize, usize, u8)> = HashSet::new();

    let l: (usize, usize, u8) = (0, 0, Orientation::East as u8);
    open_locations.push(l);
    laser_cache.insert(l);

    while !open_locations.is_empty() {
        let (r , c, dir): (usize, usize, u8) = open_locations.pop().unwrap();
        visited_locations.insert((r, c));

        let a: Vec<(usize, usize, u8)> = get_next_location(grid.clone(), r , c, dir);
        for (new_r, new_c, new_dir) in a {
            if !laser_cache.contains(&(new_r, new_c, new_dir)) && 
                    new_r < grid.len() && new_c < grid[0].len() {
                open_locations.push((new_r, new_c, new_dir));
                laser_cache.insert((new_r, new_c, new_dir));
            }
        }
    }

    ans = visited_locations.len();
    return ans;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part2(contents: String) -> usize {
    let mut ans: usize = 0;
    let grid: Vec<Vec<u8>> = convert_string_to_vec(contents);
    let grid_width: usize = grid[0].len();
    let grid_height: usize = grid.len();
    let border_coords: Vec<(usize, usize, u8)> = get_border_vals_and_dir(grid.clone(), grid_width, grid_height);

    for (i, j, d) in border_coords {
        let mut open_locations: Vec<(usize, usize, u8)> = vec![];
        let mut visited_locations: HashSet<(usize, usize)> = HashSet::new();
        let mut laser_cache: HashSet<(usize, usize, u8)> = HashSet::new();

        let l: (usize, usize, u8) = (0, 0, d);
        open_locations.push(l);
        laser_cache.insert(l);

        while !open_locations.is_empty() {
            let (r , c, dir): (usize, usize, u8) = open_locations.pop().unwrap();
            visited_locations.insert((r, c));

            let a: Vec<(usize, usize, u8)> = get_next_location(grid.clone(), r , c, dir);
            for (new_r, new_c, new_dir) in a {
                if !laser_cache.contains(&(new_r, new_c, new_dir)) && 
                        new_r < grid_height && new_c < grid_width {
                    open_locations.push((new_r, new_c, new_dir));
                    laser_cache.insert((new_r, new_c, new_dir));
                }
            }
        }
    
        if visited_locations.len() > ans {
            ans = visited_locations.len();
        }
    }

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)] #[allow(non_camel_case_types)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 46);
    }


    #[test]
    fn part2_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 51);
    }
}