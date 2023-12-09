use std::collections::HashMap;

fn main() {
    let lines = aoc2023::read_one_per_line::<String>("./data/day8.txt").unwrap();
    let ans1 = part1(&lines);
    let ans2 = part2(&lines);
    println!("PART 1 ANS: {ans1}");
    println!("PART 2 ANS: {ans2}");
}

fn part1(lines: &Vec<String>) -> u64 {
    let seq = lines[0].chars().collect::<Vec<_>>();
    let mut idx = 2;
    let mut steps = 0;
    let mut curr_pos = "AAA";
    let mut map: HashMap<String, [String; 2]> = HashMap::new();

    while idx < lines.len() {
        let (key, val_string) = lines[idx].split_once(" = ").unwrap();
        let (left, right) = val_string
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_once(", ")
            .unwrap();
        map.insert(key.to_string(), [left.to_string(), right.to_string()]);
        idx += 1;
    }

    idx = 0;

    loop {
        if curr_pos == "ZZZ" {
            break;
        }

        let next = map.get(curr_pos).unwrap();

        match seq[idx] {
            'L' => curr_pos = &next[0],
            'R' => curr_pos = &next[1],
            _ => panic!("Bad Input"),
        }

        idx = (idx + 1) % seq.len();
        steps += 1;
    }
    return steps;
}

fn part2(lines: &Vec<String>) -> u64 {
    let seq = lines[0].chars().collect::<Vec<_>>();
    let mut idx = 2;
    let mut curr_pos: Vec<&str> = vec![];
    let mut map: HashMap<&str, [&str; 2]> = HashMap::new();

    while idx < lines.len() {
        let (key, val_string) = lines[idx].split_once(" = ").unwrap();
        let (left, right) = val_string
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_once(", ")
            .unwrap();
        map.insert(key, [left, right]);
        if key.chars().last().unwrap() == 'A' {
            curr_pos.push(key);
        }
        idx += 1;
    }

    idx = 0;
    let mut final_steps = 1;

    for mut pos in curr_pos {
        let mut steps = 0;
        loop {
            if pos.chars().last().unwrap() == 'Z' {
                break;
            }

            let next = map.get(pos).unwrap();

            match seq[idx] {
                'L' => pos = &next[0],
                'R' => pos = &next[1],
                _ => panic!("Bad Input"),
            }

            idx = (idx + 1) % seq.len();
            steps += 1;
        }
        final_steps = lcm(steps, final_steps);
    }

    return final_steps;
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
