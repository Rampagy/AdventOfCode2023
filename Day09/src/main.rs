use std::fs;


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}


#[warn(non_snake_case)]
fn part1(contents: String) -> i128 {
    let mut ans: i128 = 0;
    let mut diffs: Vec<Vec<i64>> = Vec::new();
    let mut final_vals: Vec<i128> = Vec::new();

    for (_line_num, line) in contents.lines().enumerate() {
        let vals: Vec<i64> = line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

        diffs.push(vals);
        let mut loop_counter: usize = 0;
        loop {
            let diff: Vec<i64> = diffs.get(loop_counter).unwrap().to_vec();
            let mut new_diffs: Vec<i64> = Vec::new();

            for idx in 1..diff.len() {
                new_diffs.push(diff[idx] - diff[idx - 1]);
            }

            if !new_diffs.iter().all(|x| *x == 0) {
                diffs.push(new_diffs);
                loop_counter += 1;
            } else /* got to the end */ {
                new_diffs.push(0);
                diffs.push(new_diffs);
                break;
            }
        }

        /* now loop through add the one from the layer below to the current layer */
        loop_counter = diffs.len()-1;
        while loop_counter > 0 {
            let diff: i64 = *diffs.get(loop_counter).unwrap().to_vec().last().unwrap();
            let prev_val: i64 = *diffs.get(loop_counter-1).unwrap().last().unwrap();
            diffs[loop_counter-1].push(prev_val + diff);
            loop_counter -= 1;
        }

        final_vals.push(*diffs.first().unwrap().last().unwrap() as i128);
        diffs.clear();
    }

    for f in final_vals.clone() {
        ans += f;
    }

    return ans;
}


#[warn(non_snake_case)]
fn part2(contents: String) -> i128 {
    let mut ans: i128 = 0;
    let mut diffs: Vec<Vec<i64>> = Vec::new();
    let mut final_vals: Vec<i128> = Vec::new();

    for (_line_num, line) in contents.lines().enumerate() {
        let mut vals: Vec<i64> = line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

        vals.reverse();
        diffs.push(vals);
        let mut loop_counter: usize = 0;
        loop {
            let diff: Vec<i64> = diffs.get(loop_counter).unwrap().to_vec();
            let mut new_diffs: Vec<i64> = Vec::new();

            for idx in 1..diff.len() {
                new_diffs.push(diff[idx] - diff[idx - 1]);
            }

            if !new_diffs.iter().all(|x| *x == 0) {
                diffs.push(new_diffs);
                loop_counter += 1;
            } else /* got to the end */ {
                new_diffs.push(0);
                diffs.push(new_diffs);
                break;
            }
        }

        /* now loop through add the one from the layer below to the current layer */
        loop_counter = diffs.len()-1;
        while loop_counter > 0 {
            let diff: i64 = *diffs.get(loop_counter).unwrap().to_vec().last().unwrap();
            let prev_val: i64 = *diffs.get(loop_counter-1).unwrap().last().unwrap();
            diffs[loop_counter-1].push(prev_val + diff);
            loop_counter -= 1;
        }

        final_vals.push(*diffs.first().unwrap().last().unwrap() as i128);
        diffs.clear();
    }

    for f in final_vals.clone() {
        ans += f;
    }

    return ans;
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 114);
    }

    #[test]
    fn test1_part2() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 2);
    }
}