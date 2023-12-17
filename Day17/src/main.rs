extern crate priority_queue;
extern crate ordered_float;

mod position;

use std::fs;
use std::collections::{HashSet, HashMap};
use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;
use position::{Position, PositionBuildHasher};


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
            temp.push(ch as u8 - 48);
        }
        grid.push(temp);
    }

    return grid;
}


#[inline]
fn optimized_heuristic(a: Position, b: Position) -> f32 {
    return ((a.x - b.x).abs() + (a.y - b.y).abs()) as f32;
}


#[allow(non_snake_case)]
pub fn optimized_dijkstras_search(  weighted_map: &Vec<Vec<u8>>, start: Position, 
                                    goal: Position ) -> Vec<Position> {
    let mapWidth: usize = weighted_map[0].len();
    let mapHeight: usize = weighted_map.len();

    let mut path: Vec<Position> = Vec::with_capacity(1 as usize);
    if start.x < 0 || start.y < 0 || goal.x >= mapWidth as i32 || goal.y >= mapHeight as i32 ||
       start == goal || mapWidth < 2 || mapHeight < 2 {
        return path;
    }

    /* Memory allocation */
    let mut close_set: HashSet<Position, PositionBuildHasher> = HashSet::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
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


        /* check to see how many of the previous locations have been in the same direction */
        let mut trace_back_path: Vec<Position> = Vec::new();
        let mut trace_back_position: Position = current;
        let mut loop_counter: usize = 0;
        let (mut same_x, mut same_y): (u8, u8) = (0, 0);
        let mut illegal_neighbor: Position = Position::new(-1, -1);
        let mut disqualify_illegal_neighbor: bool = false;
        while trace_back_position != start && loop_counter < 4 {
            trace_back_path.push(trace_back_position);
            trace_back_position = *came_from.get(&trace_back_position).unwrap_or(&Position::new(0,0));
            loop_counter += 1;

            if trace_back_position.x == current.x {
                same_x += 1;
            }
            if trace_back_position.y == current.y {
                same_y += 1;
            }
        }

        if ((same_x >= 3 || same_y >= 3) && trace_back_position != Position::new(0, 0)) ||
                ((same_x >= 2 || same_y >= 2) && trace_back_position == Position::new(0, 0)) {
            /* cannot continue in the same direction, so disqualify any neighbor that continues same direction */
            disqualify_illegal_neighbor = true;
            illegal_neighbor.x = current.x + trace_back_path[0].x - trace_back_path[1].x;
            illegal_neighbor.y = current.y + trace_back_path[0].y - trace_back_path[1].y;
        }


        /* Search surrounding neighbors */
        for neighbor in neighbors {
            /* if the neighbor is a valid position */
            if neighbor.x >= 0 && neighbor.y >= 0 && 
                    neighbor.y < mapHeight as i32 && neighbor.x < mapWidth as i32 &&
                    weighted_map[neighbor.y as usize][neighbor.x as usize] < 255 && 
                    (!disqualify_illegal_neighbor || neighbor != illegal_neighbor) {
                let neighbor_gscore: f32 = *gscore.get(&current).unwrap_or(&0.0) + weighted_map[neighbor.y as usize][neighbor.x as usize] as f32 + 
                                            optimized_heuristic(neighbor, current);

                /* if the neighbor is already on the open list check to see if the neighbor is better before updating it*/
                let in_open_list: bool = oheap_copy.contains_key(&neighbor);
                if in_open_list && neighbor_gscore < *gscore.get(&neighbor).unwrap_or(&0.0) {
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

    /* trace path back from the goal */
    current = goal;
    while current != start {
        path.push(current);
        current = *came_from.get(&current).unwrap_or(&Position::new(0,0));
    }

    path.reverse();

    return path;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part1(contents: String) -> usize {
    let mut ans: usize = 0;
    let mut grid: Vec<Vec<u8>> = convert_string_to_vec(contents);
    let map_height: usize = grid.len();
    let map_width: usize = grid[0].len();

    /* make start position unwalkable */
    grid[0][0] = 255;

    let path: Vec<Position> = optimized_dijkstras_search(&grid, Position::new(0,0), 
                                Position::new((map_width-1) as i32, (map_height-1) as i32));

    for p in path {
        println!("{}", p);
        ans += grid[p.y as usize][p.x as usize] as usize;
    }

    return ans;
}

#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part2(contents: String) -> usize {
    let mut ans: usize = 0;
    let grid: Vec<Vec<u8>> = convert_string_to_vec(contents);

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)] #[allow(non_camel_case_types)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 102);
    }


    #[test]
    fn part2_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 51);
    }
}