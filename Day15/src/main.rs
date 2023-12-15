use std::fs;
use std::collections::HashMap;
use std::ops::Add;


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part 1: {}", part1(contents.clone()));
    println!("part 2: {}", part2(contents.clone()));
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn hash_algorithm(letter: char, previous_result: usize) -> usize {
    return (previous_result + letter as usize) * 17 % 256;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part1(contents: String) -> usize {
    let mut ans: usize = 0;

    /* get the strings */
    let hash_steps: Vec<&str> = contents.split(',').collect();

    for hash_step in hash_steps {
        let mut current_hash: usize = 0;
        for ch in hash_step.chars() {
            current_hash = hash_algorithm(ch, current_hash);
        }
        ans += current_hash;
    }

    return ans;
}


#[allow(non_snake_case)] #[allow(non_camel_case_types)]
fn part2(contents: String) -> usize {
    let mut ans: usize = 0;
    let mut lens_boxes: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

    /* get the strings */
    let hash_steps: Vec<&str> = contents.split(',').collect();

    for hash_step in hash_steps {
        let mut current_hash: usize = 0;
        let mut lens_id: String = "".to_string();
        let lens_length: usize =    if hash_step.contains('=') {
                                        hash_step.split('=').nth(1).unwrap().parse::<usize>().unwrap()
                                    } else { 0 };

        for ch in hash_step.chars() {
            if ch == '-' {
                /* remove from box if in box */
                if lens_boxes.contains_key(&current_hash) {
                    let mut lens_idx: usize = 0;
                    for (l_id, _ ) in lens_boxes.get(&current_hash).unwrap().clone() {
                        if lens_id == l_id {
                            let mut v: Vec<(String, usize)> = lens_boxes.get(&current_hash).unwrap().clone();
                            v.remove(lens_idx);
                            lens_boxes.insert(current_hash, v);
                            break;
                        }
                        lens_idx += 1
                    }
                }
                break;
            } else if ch == '=' {
                /* add to box or replace in the box if in box */
                if lens_boxes.contains_key(&current_hash) {
                    /* replace lens if already in box */
                    let mut lens_idx: usize = 0;
                    let mut in_box: bool = false;
                    for (l_id, _ ) in lens_boxes.get(&current_hash).unwrap().clone() {
                        if lens_id == l_id {
                            let mut v: Vec<(String, usize)> = lens_boxes.get(&current_hash).unwrap().clone();
                            v[lens_idx] = (l_id, lens_length);
                            lens_boxes.insert(current_hash, v);
                            in_box = true;
                            break;
                        }
                        lens_idx += 1
                    }

                    if !in_box {
                        let mut v: Vec<(String, usize)> = lens_boxes.get(&current_hash).unwrap().clone();
                        v.push((lens_id, lens_length));
                        lens_boxes.insert(current_hash, v);
                    }
                } else {
                    /* add lens to the box */
                    lens_boxes.insert(current_hash, vec![(lens_id, lens_length)]);
                }
                break;
            } else {
                current_hash = hash_algorithm(ch, current_hash);
            }

            lens_id = lens_id.add(ch.to_string().as_str());
        }
    }

    /* calcualte answer */
    for (box_num, lenses) in lens_boxes {
        let mut lens_idx: usize = 1;
        for (_lens, focal_length) in lenses {
            ans += (box_num+1) * lens_idx * focal_length;
            lens_idx += 1;
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
        assert_eq!(part1(contents.clone()), 1320);
    }


    #[test]
    fn hashalgo_test1() {
        let mut current_hash: usize = 0;
        current_hash = hash_algorithm('r', current_hash);
        current_hash = hash_algorithm('n', current_hash);
        assert_eq!(current_hash, 0);
    }


    #[test]
    fn hashalgo_test2() {
        let mut current_hash: usize = 0;
        current_hash = hash_algorithm('q', current_hash);
        current_hash = hash_algorithm('p', current_hash);
        assert_eq!(current_hash, 1);
    }


    #[test]
    fn hashalgo_test3() {
        let mut current_hash: usize = 0;
        current_hash = hash_algorithm('c', current_hash);
        current_hash = hash_algorithm('m', current_hash);
        assert_eq!(current_hash, 0);
    }


    #[test]
    fn hashalgo_test4() {
        let mut current_hash: usize = 0;
        current_hash = hash_algorithm('p', current_hash);
        current_hash = hash_algorithm('c', current_hash);
        assert_eq!(current_hash, 3);
    }


    #[test]
    fn part2_test1() {
        let contents: String = fs::read_to_string("src/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), 145);
    }
}