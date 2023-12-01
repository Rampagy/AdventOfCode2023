use std::fs;

#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    part1(contents.clone());
    part2(contents.clone());
}


fn part1(contents: String) {
    let mut numbers: Vec<u8> = Vec::new();
    for line in contents.lines() {
        let mut num_as_str: String = String::new();
        for letter in line.chars() {
            if letter >= '1' && letter <= '9' {
                num_as_str.push(letter);
                break;
            }
        }

        for letter in line.chars().rev() {
            if letter >= '1' && letter <= '9' {
                num_as_str.push(letter);
                break;
            }
        }

        let num: u8 = num_as_str.parse().unwrap();
        numbers.push(num);
    }

    let mut sum: u32 = 0;
    for num in numbers {
        sum += num as u32;
    }

    println!("{}", sum);
}


fn part2(contents: String) {
    let mut numbers: Vec<u8> = Vec::new();
    for line in contents.lines() {
        let mut num_as_str: String = String::new();
        for (i, letter) in line.chars().enumerate() {
            if letter >= '1' && letter <= '9' {
                num_as_str.push(letter);
                break;
            } else if line[i..].starts_with("one") {
                num_as_str.push('1');
                break;
            } else if line[i..].starts_with("two") {
                num_as_str.push('2');
                break;
            } else if line[i..].starts_with("three") {
                num_as_str.push('3');
                break;
            } else if line[i..].starts_with("four") {
                num_as_str.push('4');
                break;
            } else if line[i..].starts_with("five") {
                num_as_str.push('5');
                break;
            } else if line[i..].starts_with("six") {
                num_as_str.push('6');
                break;
            } else if line[i..].starts_with("seven") {
                num_as_str.push('7');
                break;
            } else if line[i..].starts_with("eight") {
                num_as_str.push('8');
                break;
            } else if line[i..].starts_with("nine") {
                num_as_str.push('9');
                break;
            }
        }

        for (i, letter) in line.chars().rev().enumerate() {
            if letter >= '1' && letter <= '9' {
                num_as_str.push(letter);
                break;
            } else {
                let reversed_line: String = line.chars().rev().collect::<String>();

                if reversed_line[i..].starts_with("eno") {
                    num_as_str.push('1');
                    break;
                } else if reversed_line[i..].starts_with("owt") {
                    num_as_str.push('2');
                    break;
                } else if reversed_line[i..].starts_with("eerht") {
                    num_as_str.push('3');
                    break;
                } else if reversed_line[i..].starts_with("ruof") {
                    num_as_str.push('4');
                    break;
                } else if reversed_line[i..].starts_with("evif") {
                    num_as_str.push('5');
                    break;
                } else if reversed_line[i..].starts_with("xis") {
                    num_as_str.push('6');
                    break;
                } else if reversed_line[i..].starts_with("neves") {
                    num_as_str.push('7');
                    break;
                } else if reversed_line[i..].starts_with("thgie") {
                    num_as_str.push('8');
                    break;
                } else if reversed_line[i..].starts_with("enin") {
                    num_as_str.push('9');
                    break;
                }
            }
        }

        let num: u8 = num_as_str.parse().unwrap();
        numbers.push(num);
    }

    let mut sum: u32 = 0;
    for num in numbers {
        sum += num as u32;
    }

    println!("{}", sum);
}