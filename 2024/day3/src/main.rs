use regex::Regex;
use utils::read_file;

fn main() {
    let result = read_file(String::from("input.txt"));
    part1(&result);
    part2(&result);
}

fn part1(result: &str) {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let instructions: Vec<&str> = re.find_iter(&result).map(|mat| mat.as_str()).collect();
    let re = Regex::new(r"\d+,\d+").unwrap();
    let mut ans = 0;
    for instruction in instructions {
        let numbers_vec: Vec<&str> = re.find_iter(&instruction).map(|mat| mat.as_str()).collect();
        let numbers: Vec<i32> = numbers_vec[0]
            .split(',')
            .map(|x| x.parse().expect("Error parsing"))
            .collect();
        ans += numbers[0] * numbers[1];
    }
    println!("{}", ans);
}

fn part2(result: &str) {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let instructions: Vec<&str> = re.find_iter(&result).map(|mat| mat.as_str()).collect();
    let re = Regex::new(r"\d+,\d+").unwrap();
    let mut ans = 0;
    let mut enable = true;
    for instruction in instructions {
        if instruction == "don't()" {
            enable = false;
            continue;
        } else if instruction == "do()" {
            enable = true;
            continue;
        }

        if !enable {
            continue;
        }

        let numbers_vec: Vec<&str> = re.find_iter(&instruction).map(|mat| mat.as_str()).collect();
        let numbers: Vec<i32> = numbers_vec[0]
            .split(',')
            .map(|x| x.parse().expect("Error parsing"))
            .collect();
        ans += numbers[0] * numbers[1];
    }
    println!("{}", ans);
}
