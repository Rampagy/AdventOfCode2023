mod position;

use std::fs;
use std::collections::{HashSet, HashMap};
use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;
use position::{Position, PositionBuildHasher};


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone(), 124));
    println!("part 2: {}", part2(contents.clone(), 124));
}


#[allow(non_snake_case)]
pub fn search_map(weighted_map: &Vec<Vec<u8>>, start: Position) -> HashSet<Position, PositionBuildHasher> {
    let mapWidth: usize = weighted_map[0].len();
    let mapHeight: usize = weighted_map.len();

    let mut close_set: HashSet<Position, PositionBuildHasher> = HashSet::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);

    let mut path: Vec<Position> = Vec::with_capacity(1);
    if start.x < 0 || start.y < 0 || mapWidth < 2 || mapHeight < 2 {
        return close_set;
    }

    /* Memory allocation */
    let mut came_from: HashMap<Position, Position, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut gscore: HashMap<Position, f32, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut oheap: PriorityQueue<Position, OrderedFloat<f32>, PositionBuildHasher> = PriorityQueue::with_capacity_and_hasher(mapWidth + mapHeight, PositionBuildHasher);
    let mut oheap_copy: HashMap<Position, f32, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);

    let mut current: Position;
    let mut neighbors: [Position; 4];

    /* Add initial position to the search list */
    gscore.insert(start, 0.0);

    /* Note: gscore is multiplied by -1 before being entered into the oheap
     *  because of how big of a pain in the ass it is to switch it from a
     *  max heap to a min heap */
    oheap.push(start, OrderedFloat::from(-1.0*(*gscore.get(&start).unwrap_or(&0.0))));
    oheap_copy.insert(start, *gscore.get(&start).unwrap_or(&0.0));

    let mut _count: u32 = 0;
    while !oheap.is_empty() {
        _count += 1;
        (current, _) = oheap.pop().unwrap_or((Position::new(0,0), OrderedFloat::from(0.0)));
        oheap_copy.remove(&current);
        close_set.insert(current);

        neighbors = current.get_surrounding_positions();

        /* Search surrounding neighbors */
        for neighbor in neighbors {
            /* if the neighbor is a valid position */
            if neighbor.x >= 0 && neighbor.y >= 0 && 
                    neighbor.y < mapHeight as i32 && neighbor.x < mapWidth as i32 &&
                    is_valid_pipe_neighbor(weighted_map[current.y as usize][current.x as usize], weighted_map[neighbor.y as usize][neighbor.x as usize], current, neighbor) {
                let neighbor_gscore: f32 = *gscore.get(&current).unwrap_or(&0.0) + optimized_heuristic(neighbor, current);

                /* if the neighbor is already on the open list check to see if the neighbor is better before updating it*/
                let in_open_list: bool = oheap_copy.contains_key(&neighbor);
                if in_open_list && neighbor_gscore < *gscore.get(&neighbor).unwrap_or(&0.0){
                    /* track the node's parent */
                    came_from.insert(neighbor, current);

                    /* gscore = cost to get from the start to the current position */
                    gscore.entry(neighbor).and_modify(|val| *val = neighbor_gscore);

                    /* update the neighbors values */
                    oheap_copy.entry(neighbor).and_modify(|val| *val = neighbor_gscore);

                    /* remove the old gscore */
                    oheap.remove(&neighbor);

                    /* Add the new fscore and sort */
                    oheap.push(neighbor, OrderedFloat::from(-1.0*neighbor_gscore));
                    continue;
                }

                /* check if it is on the closed list */
                if close_set.contains(&neighbor) && neighbor_gscore < *gscore.get(&neighbor).unwrap_or(&0.0) {
                    /* remove neighbor from closed list */
                    close_set.remove(&neighbor);
                }

                /* Add to the open list */
                if !close_set.contains(&neighbor) && !in_open_list {
                    /* track the node's parent */
                    came_from.insert(neighbor, current);

                    /* gscore = cost to get rom the start to the current position */
                    gscore.insert(neighbor, neighbor_gscore);

                    /* add to the open list */
                    oheap_copy.insert(neighbor, neighbor_gscore);
                    oheap.push(neighbor, OrderedFloat::from(-1.0*neighbor_gscore));
                }
            }
        }
    }

    /* loop through g-score and find the biggest number */
    let mut path_dist: usize = 0;
    let mut goal: Position = Position::new(0, 0);
    for (l, score) in gscore.iter() {
        if *score as usize > path_dist {
            path_dist = *score as usize;
            goal = *l;
        }
    }

    return close_set;
}


fn is_valid_pipe_neighbor (current_pipe: u8, neighbor_pipe: u8, current_loc: Position, neighbor_loc: Position) -> bool {
    /* let valid_neighbor_val: bool =  match neighbor_pipe {
        124 => true, // |
        45 => true, // -
        76 => true, // L
        74 => true, // J
        55 => true, // 7
        70 => true, // F
        _ => false, 
    }; */


    let dx: i32 = current_loc.x - neighbor_loc.x;
    let dy: i32 = current_loc.y - neighbor_loc.y;

    if dx > 0 {
        // trying to go west
        if (neighbor_pipe == 76 || neighbor_pipe == 45 || neighbor_pipe == 70) && 
                (current_pipe == 74 || current_pipe == 55 || current_pipe == 45) {
            return true;
        } else {
            return false;
        }
    } else if dx < 0 {
        // east
        if (current_pipe == 76 || current_pipe == 45 || current_pipe == 70) && 
                (neighbor_pipe == 74 || neighbor_pipe == 55 || neighbor_pipe == 45) {
            return true;
        } else {
            return false;
        }
    } else {
        if dy > 0 {
            // north
            if (current_pipe == 124 || current_pipe == 76 || current_pipe == 74) && 
                    (neighbor_pipe == 124 || neighbor_pipe == 55 || neighbor_pipe == 70) {
                return true;
            } else {
                return false;
            }
         } else {
            // south
            if (current_pipe == 124 || current_pipe == 55 || current_pipe == 70) && 
                    (neighbor_pipe == 124 || neighbor_pipe == 76 || neighbor_pipe == 74) {
                return true;
            } else {
                return false;
            }
        }
    }
}


#[inline]
fn optimized_heuristic(a: Position, b: Position) -> f32 {
    return (((a.x - b.x) + (a.y - b.y)) as f32).abs();
}


#[warn(non_snake_case)]
fn part1(contents: String, start_pipe: u8) -> u64 {
    let mut ans: u64 = 0;
    let mut pipe_map: Vec<Vec<u8>> = Vec::new();
    let mut start: Position = Position::new(0, 0);

    for (line_num, line) in contents.lines().enumerate() {
        let mut row: Vec<u8> = Vec::new();
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start.x = col as i32;
                start.y = line_num as i32;

                // replace S with a proper pipe before searching the map
                row.push(start_pipe);
            } else {
                row.push(ch as u8);
            }
        }

        pipe_map.push(row);
    }

    
    pipe_map[start.y as usize][start.x as usize] = start_pipe;
    let path: HashSet<Position, PositionBuildHasher> = search_map(&pipe_map, start);

    ans = path.len() as u64 / 2;

    return ans;
}


#[warn(non_snake_case)]
fn part2(contents: String, start_pipe: u8) -> u64 {
    let mut ans: u64 = 0;
    let mut pipe_map: Vec<Vec<u8>> = Vec::new();
    let mut start: Position = Position::new(0, 0);

    for (line_num, line) in contents.lines().enumerate() {
        let mut row: Vec<u8> = Vec::new();
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start.x = col as i32;
                start.y = line_num as i32;

                // replace S with a proper pipe before searching the map
                row.push(start_pipe);
            } else {
                row.push(ch as u8);
            }
        }

        pipe_map.push(row);
    }

    pipe_map[start.y as usize][start.x as usize] = start_pipe;
    let path: HashSet<Position, PositionBuildHasher> = search_map(&pipe_map, start);

    /* Shamelessly stole this solution from here:
     *    https://www.reddit.com/r/adventofcode/comments/18evyu9/2023_day_10_solutions/kcqtow6/ */
    let mut y: i32 = 0;
    for row in pipe_map.clone() {
        let mut x :i32 = 0;
        for ch in row {
            if !path.contains(&Position::new(x, y)) { // don't search spots that are part of the path
                // loop diagonally counting how many times the path is crossed
                let mut paths_cross: u64 = 0;
                let mut x_mod: i32 = x - 1;
                let mut y_mod: i32 = y + 1;
                while x_mod >= 0 && y_mod < pipe_map[0].len() as i32 {
                    if path.contains(&Position::new(x_mod, y_mod)) && 
                            pipe_map[y_mod as usize][x_mod as usize] != 70 && pipe_map[y_mod as usize][x_mod as usize] != 74 {
                        // count positions in the path that aren't F and J
                        paths_cross += 1;
                    }

                    // loop in the southwest direction
                    x_mod -= 1;
                    y_mod += 1;
                }

                if paths_cross % 2 == 1 {
                    // if the paths crossed an odd number of times then it's in the loop
                    ans += 1;
                }
            }
            x += 1;
        }
        y += 1;
    }

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone(), 70), 4);
    }

    #[test]
    fn test2_part1() {
        let contents: String = fs::read_to_string("src/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone(), 70), 8);
    }

    #[test]
    fn test1_part2() {
        let contents: String = fs::read_to_string("src/test3.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), 70), 4);
    }

    #[test]
    fn test2_part2() {
        let contents: String = fs::read_to_string("src/test4.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), 70), 4);
    }

    #[test]
    fn test3_part2() {
        let contents: String = fs::read_to_string("src/test5.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), 70), 8);
    }

    #[test]
    fn test4_part2() {
        let contents: String = fs::read_to_string("src/test6.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone(), 55), 10);
    }
}