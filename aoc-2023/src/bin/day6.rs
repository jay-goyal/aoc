fn main() {
    let lines = aoc2023::read_one_per_line::<String>("./data/day6.txt").unwrap();
    let ans1 = part1(&lines);
    let ans2 = part2(&lines);
    println!("PART 1 ANS: {ans1}");
    println!("PART 2 ANS: {ans2}");
}

fn part1(lines: &Vec<String>) -> u64 {
    let times: Vec<u32> = lines[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = lines[1]
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut ans = 1;

    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let mut start = 0;
        let mut end = 0;
        for j in 1..=time {
            if j * (time - j) > distance {
                start = j as u64;
                break;
            }
        }

        for j in (1..=time).rev() {
            if j * (time - j) > distance {
                end = j as u64;
                break;
            }
        }
        ans *= u64::abs_diff(end, start) + 1;
    }
    return ans;
}

fn part2(lines: &Vec<String>) -> u64 {
    let time: u64 = lines[0]
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let distance: u64 = lines[1]
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let mut ans = 1;

    let mut start = 0;
    let mut end = 0;
    for j in 1..=time {
        if j * (time - j) > distance {
            start = j;
            break;
        }
    }

    for j in (1..=time).rev() {
        if j * (time - j) > distance {
            end = j;
            break;
        }
    }
    ans *= u64::abs_diff(end, start) + 1;
    return ans;
}
