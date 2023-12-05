use std::collections::HashSet;

fn main() {
    let lines = aoc2023::read_one_per_line::<String>("./data/day5.txt").unwrap();
    let ans1 = part1(&lines);
    let ans2 = part2(&lines);
    println!("PART 1 ANS: {ans1}");
    println!("PART 2 ANS: {ans2}");
}

fn part1(lines: &Vec<String>) -> u64 {
    let mut lines_iter = lines.iter();
    let seeds: HashSet<u64> = lines_iter
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut maps: Vec<Vec<[u64; 3]>> = Vec::new();
    let mut pass_next = false;
    for line in lines_iter {
        // Ignore the line with text
        if pass_next {
            pass_next = false;
            continue;
        }

        if line.is_empty() {
            maps.push(Vec::new());
            pass_next = true;
            continue;
        }

        let val = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let l = maps.len();
        maps[l - 1].push([val[0], val[1], val[2]]);
    }

    let mut ans = u64::MAX;

    for seed in seeds {
        let mut val = seed;
        for map in &maps {
            for m in map {
                if val >= m[1] && val < m[1] + m[2] {
                    val = m[0] + val - m[1];
                    break;
                }
            }
        }
        ans = ans.min(val);
    }

    ans
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut lines_iter = lines.iter();
    let seeds: HashSet<[u64; 2]> = lines_iter
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|x| [x[0], x[1]])
        .collect();

    let mut maps: Vec<Vec<[u64; 3]>> = Vec::new();
    let mut pass_next = false;
    for line in lines_iter {
        // Ignore the line with text
        if pass_next {
            pass_next = false;
            continue;
        }

        if line.is_empty() {
            maps.push(Vec::new());
            pass_next = true;
            continue;
        }

        let val = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let l = maps.len();
        maps[l - 1].push([val[0], val[1], val[2]]);
    }

    let mut ans = u64::MAX;

    for seed_data in seeds {
        let seed_base = seed_data[0];
        for seed_off in 0..seed_data[1] {
            let mut val = seed_base + seed_off;
            for map in &maps {
                for m in map {
                    if val >= m[1] && val < m[1] + m[2] {
                        val = m[0] + val - m[1];
                        break;
                    }
                }
            }
            ans = ans.min(val);
        }
    }

    ans
}
