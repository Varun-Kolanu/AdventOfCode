use utils::read_file;

fn main() {
    let result = read_file(String::from("input.txt"));
    let lines: Vec<&str> = result.split('\n').collect();

    let mut differences = Vec::new();
    for line in lines {
        let levels: Vec<i32> = line
            .split(' ')
            .map(|x| x.parse().expect("Error parsing"))
            .collect();

        let mut difference_vec = Vec::new();
        for i in 0..levels.len() - 1 {
            difference_vec.push(levels[i] - levels[i + 1]);
        }
        differences.push(difference_vec);
    }
    part1(&differences);
    part2(&differences);
}

fn part1(differences: &Vec<Vec<i32>>) {
    let mut safe = 0;
    for difference in differences {
        if difference_safe(difference) {
            safe += 1;
        }
    }
    println!("{}", safe);
}

fn part2(differences: &Vec<Vec<i32>>) {
    let mut safe = 0;
    for difference in differences {
        let length = difference.len();
        if difference_safe(difference) {
            safe += 1;
            continue;
        }

        let mut broke = false;
        for i in 0..length {
            let mut new_difference = difference.clone();
            if i > 0 {
                new_difference[i - 1] += new_difference[i];
            }
            new_difference.remove(i);
            if difference_safe(&new_difference) {
                safe += 1;
                broke = true;
                break;
            }
        }
        if !broke {
            let mut new_difference = difference.clone();
            new_difference.remove(length - 1);
            if difference_safe(&new_difference) {
                safe += 1;
            }
        }
    }
    println!("{}", safe);
}

fn adj_difference_safe(diff: i32) -> bool {
    return diff >= 1 && diff <= 3;
}

fn difference_safe(difference: &Vec<i32>) -> bool {
    let length = difference.len();
    let mut broke = false;
    for i in 0..length - 1 {
        if difference[i] * difference[i + 1] <= 0 || !adj_difference_safe(difference[i].abs()) {
            broke = true;
            break;
        }
    }
    if !adj_difference_safe(difference[length - 1].abs()) {
        broke = true;
    }

    return !broke;
}
