fn main() {
    let lines = aoc2023::read_one_per_line::<String>("./data/day9.txt").unwrap();
    let ans1 = part1(&lines);
    let ans2 = part2(&lines);
    println!("PART 1 ANS: {ans1}");
    println!("PART 2 ANS: {ans2}");
}

fn part1(lines: &Vec<String>) -> i64 {
    let mut ans = 0;
    for line in lines {
        let mut seq = vec![line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()];
        let mut last = seq.last().unwrap();
        while last.iter().sum::<i64>() != 0 {
            let mut seq_push = Vec::new();
            for i in 0..(last.len() - 1) {
                seq_push.push(last[i + 1] - last[i])
            }

            seq.push(seq_push);
            last = seq.last().unwrap();
        }

        let mut next = 0;
        let len = seq.len();

        for i in (0..len).rev() {
            seq[i].push(next);
            if i > 0 {
                next = seq[i - 1].last().unwrap() + seq[i].last().unwrap();
            }
        }

        ans += seq[0].last().unwrap();
    }
    return ans;
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut ans = 0;
    for line in lines {
        let mut seq = vec![line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()];
        let mut last = seq.last().unwrap();
        while last.iter().sum::<i64>() != 0 {
            let mut seq_push = Vec::new();
            for i in 0..(last.len() - 1) {
                seq_push.push(last[i + 1] - last[i])
            }

            seq.push(seq_push);
            last = seq.last().unwrap();
        }

        let mut next = 0;
        let len = seq.len();

        for i in (0..len).rev() {
            seq[i].insert(0, next);
            if i > 0 {
                next = seq[i - 1].first().unwrap() - seq[i].first().unwrap();
            }
        }

        ans += seq[0].first().unwrap();
    }
    return ans;
}
