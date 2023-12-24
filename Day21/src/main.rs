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

    println!("part 1: {}", part1(contents.clone(), 64));
    println!("part 2: {}", part2(contents.clone(), 26501365));
}


#[allow(non_snake_case)]
fn convert_string_to_vec(text: String) -> (Vec<Vec<u8>>, Position) {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut start: Position = Position::new(0, 0);

    for (i, line) in text.lines().enumerate() {
        let mut temp: Vec<u8> = vec![];
        for (j, ch) in line.chars().enumerate() {
            temp.push( match ch {
                '#' => 255,
                '.' => 0,
                'S' => {
                    start.x = i as i32;
                    start.y = j as i32;
                    0
                },
                _ => {
                    panic!();
                },
            });
        }
        grid.push(temp);
    }

    return (grid, start);
}


#[allow(non_snake_case)]
pub fn optimized_dijkstras_search(weighted_map: &Vec<Vec<u8>>, start: Position, 
                                  depth_limit: usize ) -> usize {
    let mapWidth: usize = weighted_map[0].len();
    let mapHeight: usize = weighted_map.len();

    let mut path: Vec<Position> = Vec::with_capacity(1 as usize);
    if start.x < 0 || start.y < 0 || mapWidth < 2 || mapHeight < 2 {
        return usize::MAX;
    }

    /* Memory allocation */
    let mut close_set: HashSet<Position, PositionBuildHasher> = HashSet::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut came_from: HashMap<Position, Position, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut gscore: HashMap<Position, f32, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut oheap: PriorityQueue<Position, OrderedFloat<f32>, PositionBuildHasher> = PriorityQueue::with_capacity_and_hasher(mapWidth + mapHeight, PositionBuildHasher);
    let mut oheap_copy: HashMap<Position, f32, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut garden_plots: HashSet<Position, PositionBuildHasher> = HashSet::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);

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

        /* trace from current to start and see how many steps we've gone */
        let mut trace_back: Position = current.clone();
        path.clear();
        while trace_back != start {
            path.push(trace_back);
            trace_back = *came_from.get(&trace_back).unwrap_or(&Position::new(0,0));
        }

        if path.len() <= depth_limit && path.len() % 2 == depth_limit % 2 && !garden_plots.contains(&current){
            garden_plots.insert(current);
        }

        if path.len() <= depth_limit {
            neighbors = current.get_surrounding_positions();

            /* Search surrounding neighbors */
            for neighbor in neighbors {
                /* if the neighbor is a valid position */
                let wrapped_x: i32 = modified_modulus(neighbor.x, mapWidth as i32);
                let wrapped_y: i32 = modified_modulus(neighbor.y, mapHeight as i32);

                if weighted_map[wrapped_y as usize][wrapped_x as usize] < 255 {
                    let neighbor_gscore: f32 = *gscore.get(&current).unwrap_or(&0.0) + weighted_map[wrapped_y as usize][wrapped_x as usize] as f32 + 
                                                optimized_heuristic(neighbor, current);

                    /* if the neighbor is already on the open list check to see if the neighbor is better before updating it */
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
    }

    return garden_plots.len();
}


#[inline]
fn optimized_heuristic(a: Position, b: Position) -> f32 {
    return ((a.x - b.x).abs() + (a.y - b.y).abs()) as f32;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part1(contents: String, depth_limit: usize) -> usize {
    let mut ans: usize = 0;
    let (grid, start): (Vec<Vec<u8>>, Position) = convert_string_to_vec(contents.clone());

    ans = optimized_dijkstras_search(&grid, start, depth_limit);

    return ans;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part2(contents: String, depth_limit: usize) -> usize {
    let (grid, start): (Vec<Vec<u8>>, Position) = convert_string_to_vec(contents.clone());
    let mut ans: usize = 0;

    if depth_limit > (grid.len()>>1)+(grid.len()*2) {
        // search depth is too deep, so need to extrapolate from previous answers
        let mut a: [usize; 3] = [1; 3];
        for (i, depth) in [grid.len()>>1, (grid.len()>>1)+grid.len(), (grid.len()>>1)+(grid.len()*2)].iter().enumerate() {
            a[i] = optimized_dijkstras_search(&grid, start, *depth);
        }

        let b: [usize; 3] = [a[0], a[1]-a[0], a[2]-a[1]];
        let n: usize = depth_limit / grid.len();
        println!("a0: {}, a1: {}, a2: {}, b0: {}, b1: {}, b2: {}", a[0], a[1], a[2], b[0], b[1], b[2]);
        ans = b[0] + b[1]*n + ((n*(n-1)) >> 1) * (b[2]-b[1]);
    } else {
        ans = optimized_dijkstras_search(&grid, start, depth_limit);
    }

    return ans;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn modified_modulus(a: i32, limit: i32) -> i32 {
    return ((a % limit) + limit) % limit;
}


#[cfg(test)] #[allow(non_snake_case)] #[allow(non_camel_case_types)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone(), 6), 16);
    }

    #[test]
    fn modulus_test() {
        assert_eq!(modified_modulus(9, 10), 9);
        assert_eq!(modified_modulus(-1, 10), 9);
    }

    #[test]
    fn part2_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        
        /* NOTE: My input solution does not work for the test input!
            The algorithm I used only works for the real input */
        assert_eq!(part2(contents.clone(), 6), 16);
        assert_eq!(part2(contents.clone(), 10), 50);
        assert_eq!(part2(contents.clone(), 50), 1594);
        assert_eq!(part2(contents.clone(), 100), 6536);
        assert_eq!(part2(contents.clone(), 500), 167004);
        assert_eq!(part2(contents.clone(), 1000), 668697);
        assert_eq!(part2(contents.clone(), 5000), 16733044);
    }
}