use std::{fs, thread::yield_now};


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone(), 200000000000000, 400000000000000)); // 5098 - too low, 12396 - too low
    println!("part 2: {}", part2(contents.clone()));
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part1(contents: String, test_area_min: usize, test_area_max: usize) -> usize {
    let mut ans: usize = 0;
    let mut trajectories: Vec<[f64; 9]> = Vec::new();

    for line in contents.lines() {
        let points: Vec<isize> = line.split(" @ ").nth(0).unwrap().split(", ")
                                    .map(|x| x.trim().parse::<isize>().unwrap()).collect();
        let slopes: Vec<isize> = line.split(" @ ").nth(1).unwrap().split(", ")
                                    .map(|x| x.trim().parse::<isize>().unwrap()).collect();

        let slope: f64 = slopes[1] as f64 / slopes[0] as f64;

        /* general form of a line */
        let A: f64 = 1.0;
        let B: f64 = -1.0/slope;
        let C: f64 = points[1] as f64/slope - points[0] as f64;

        trajectories.push([points[0] as f64, points[1] as f64, points[2] as f64,
                            A, B, C,
                            slopes[0] as f64, slopes[1] as f64, slopes[2] as f64]);
    }

    for i in 0..trajectories.len()-1 {
        for j in i+1..trajectories.len() {
            let (a1, b1, c1) = (trajectories[i][3], trajectories[i][4], trajectories[i][5]); 
            let (a2, b2, c2) = (trajectories[j][3], trajectories[j][4], trajectories[j][5]);

            let x_intercept: f64 = (b1*c2 - b2*c1) / (a1*b2 - a2*b1);
            let y_intercept: f64 = (c1*a2 - c2*a1) / (a1*b2 - a2*b1);

            //println!("{} {} {} :: {} {} {}", 
            //                trajectories[i][0], trajectories[i][1], trajectories[i][2],
            //                trajectories[j][0], trajectories[j][1], trajectories[j][2]);
            //println!("{} {} {} :: {} {} {}", 
            //                trajectories[i][3], trajectories[i][4], trajectories[i][5],
            //                trajectories[j][3], trajectories[j][4], trajectories[j][5]);
            //println!("{} {}", x_intercept, y_intercept);
            //println!();

            if a1*b2 - a2*b1 == 0.0 {
                /* parallel */
                continue;
            } else if x_intercept > test_area_max as f64 || x_intercept < test_area_min as f64 ||
                        y_intercept > test_area_max as f64 || y_intercept < test_area_min as f64 {
                /* invalid intersection */
                continue;
            } else if x_intercept > trajectories[i][0] && trajectories[i][6] < 0.0 || 
                    x_intercept > trajectories[j][0] && trajectories[j][6] < 0.0 {
                /* x intersection is in the past */
                continue;
            } else if x_intercept < trajectories[i][0] && trajectories[i][6] > 0.0 || 
                    x_intercept < trajectories[j][0] && trajectories[j][6] > 0.0 {
                /* x intersection is in the past */
                continue;
            } else if y_intercept > trajectories[i][1] && trajectories[i][7] < 0.0 || 
                    y_intercept > trajectories[j][1] && trajectories[j][7] < 0.0 {
                /* y intersection is in the past */
                continue;
            } else if y_intercept < trajectories[i][1] && trajectories[i][7] > 0.0 || 
                    y_intercept < trajectories[j][1] && trajectories[j][7] > 0.0 {
                /* y intersection is in the past */
                continue;
            } else {
                ans += 1;
            }

            /*
            if trajectories[i][6] > 0.0 && trajectories[j][6] > 0.0 {
                if x_intercept >= trajectories[i][0] && x_intercept >= trajectories[j][0] {
                    ans += 1;
                    println!("a");
                }
            } else if trajectories[i][6] > 0.0 && trajectories[j][6] < 0.0 {
                if x_intercept >= trajectories[i][0] && x_intercept <= trajectories[j][0] {
                    ans+= 1;
                    println!("b");
                }
            } else if trajectories[i][6] < 0.0 && trajectories[j][6] < 0.0 {
                if x_intercept <= trajectories[i][0] && x_intercept <= trajectories[j][0] {
                    ans += 1;
                    println!("c");
                }
            }*/
        }
    }

    return ans;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part2(contents: String) -> usize {
    let mut ans: usize = 0;


    return ans;
}


#[cfg(test)] #[allow(non_snake_case)] #[allow(non_camel_case_types)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone(), 7, 27), 2);
    }


    #[test]
    fn part2_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 16);
    }
}