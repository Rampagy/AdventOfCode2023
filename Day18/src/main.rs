mod position;

use std::fs;
use position::Position;


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}


#[allow(non_snake_case)]
fn shoelace_algorithm(mut points: Vec<Position>) -> usize {
    /* https://www.101computing.net/the-shoelace-algorithm/ */

    points.reverse();
    let mut accumulator: isize = 0;
    for i in 0..points.len() {
        let next_idx: usize = (i+1) % points.len();

        let x1 = points[i].x as isize;
        let y1 = points[i].y as isize;
        let x2 = points[next_idx].x as isize;
        let y2 = points[next_idx].y as isize;

        accumulator += x1*y2 - x2*y1;
    }

    return (accumulator/2).abs() as usize;
}


#[allow(non_snake_case)]
fn visualize(points: Vec<Position>) {
    let mut map: Vec<Vec<char>> = vec![vec!['.'; 15]; 15];

    for p in points {
        map[p.y as usize][p.x as usize] = '#'
    }

    for r in map {
        for c in r {
            print!("{}", c);
        }
        println!("");
    }
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part1(contents: String) -> usize {
    let mut ans: usize = 0;
    let mut points: Vec<Position> = vec![Position::new(0, 0)];
    let mut perimeter: usize = 0;

    for line in contents.lines() {
        let direction: &str = line.split_ascii_whitespace().nth(0).unwrap();
        let distance: usize = line.split_ascii_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
        perimeter += distance;

        points.push( match direction {
            "R" => {
                Position::new(points.last().unwrap().x + distance as i32, points.last().unwrap().y)
            },
            "D" => {
                Position::new(points.last().unwrap().x, points.last().unwrap().y + distance as i32)
            },
            "L" => {
                Position::new(points.last().unwrap().x - distance as i32, points.last().unwrap().y)
            },
            "U" => {
                Position::new(points.last().unwrap().x, points.last().unwrap().y - distance as i32)
            },
            _ =>  {
                panic!("crash and burn!");
            }
        });
    }

    points.pop(); // remove the last value to prevent duplicate points at the start and end
    ans = shoelace_algorithm(points.clone()) + perimeter/2 + 1;
    return ans;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part2(contents: String) -> usize {
    let mut ans: usize = 0;
    let mut points: Vec<Position> = vec![Position::new(0, 0)];
    let mut perimeter: usize = 0;

    for line in contents.lines() {
        let temp: &str = line.split_ascii_whitespace().nth(2).unwrap().strip_prefix("(#").unwrap().strip_suffix(")").unwrap();
        let direction: char = temp.chars().last().unwrap();
        let distance: usize = usize::from_str_radix(&temp[0..temp.len()-1], 16).unwrap();
        perimeter += distance;

        points.push( match direction {
            '0' => {
                Position::new(points.last().unwrap().x + distance as i32, points.last().unwrap().y)
            },
            '1' => {
                Position::new(points.last().unwrap().x, points.last().unwrap().y + distance as i32)
            },
            '2' => {
                Position::new(points.last().unwrap().x - distance as i32, points.last().unwrap().y)
            },
            '3' => {
                Position::new(points.last().unwrap().x, points.last().unwrap().y - distance as i32)
            },
            _ =>  {
                panic!("crash and burn!");
            }
        });
    }

    points.pop(); // remove the last value to prevent duplicate points at the start and end
    ans = shoelace_algorithm(points.clone()) + perimeter/2 + 1;
    return ans;
}


#[cfg(test)] #[allow(non_snake_case)] #[allow(non_camel_case_types)]
mod tests {
    use super::*;

    #[test]
    fn shoelace_test1() {
        let points: Vec<Position> = vec![
            Position::new(2,7),
            Position::new(10,1),
            Position::new(8,6),
            Position::new(11,7),
            Position::new(7,10),
        ];
        assert_eq!(shoelace_algorithm(points), 32);
    }


    #[test]
    fn part1_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 62);
    }


    #[test]
    fn part2_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 952408144115);
    }
}