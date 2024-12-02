use std::collections::HashMap;

use utils::read_file;

fn main() {
    let result = read_file(String::from("input.txt"));
    let lines: Vec<&str> = result.split('\n').collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut group1: Vec<i32> = Vec::new();
    let mut group2: Vec<i32> = Vec::new();
    for line in lines {
        let location_ids: Vec<&str> = line.split("   ").collect();
        group1.push(location_ids[0].parse().expect("Error parsing"));
        group2.push(location_ids[1].parse().expect("Error parsing"));
    }

    group1.sort();
    group2.sort();
    let mut distance = 0;
    for i in 0..group1.len() {
        let dist = group1[i] - group2[i];
        distance += dist.abs();
    }
    println!("{}", distance);
}

fn part2(lines: &Vec<&str>) {
    let mut group2_counts: HashMap<&str, i32> = HashMap::new();
    for line in lines.iter() {
        let location_ids: Vec<&str> = line.split("   ").collect();
        if let Some(val) = group2_counts.get_mut(location_ids[1]) {
            *val += 1;
        } else {
            group2_counts.insert(location_ids[1], 1);
        }
    }

    let mut ans = 0;
    for line in lines.iter() {
        let location_ids: Vec<&str> = line.split("   ").collect();
        if let Some(reps) = group2_counts.get(location_ids[0]) {
            let parsed_value: i32 = location_ids[0].parse().expect("Error parsing");
            ans += parsed_value * *reps;
        }
    }
    println!("{}", ans);
}
