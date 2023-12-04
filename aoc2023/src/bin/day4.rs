fn main() {
    let lines = aoc2023::read_one_per_line::<String>("./data/day4.txt").unwrap();
    let ans1 = part1(&lines);
    let ans2 = part2(&lines);
    println!("PART 1 ANS: {ans1}");
    println!("PART 2 ANS: {ans2}");
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in lines {
        let mut card_val = 0;
        let card = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let wins: Vec<u32> = card
            .0
            .trim()
            .replace("  ", " ")
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let conts: Vec<u32> = card
            .1
            .trim()
            .replace("  ", " ")
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        for win_num in wins {
            if conts.contains(&win_num) {
                if card_val == 0 {
                    card_val += 1;
                } else {
                    card_val *= 2;
                }
            }
        }
        sum += card_val;
    }
    sum
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut cards: Vec<u32> = Vec::new();
    let mut card_copies: Vec<u64> = Vec::new();
    for line in lines {
        let mut card_val = 0;
        let card = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let wins: Vec<u32> = card
            .0
            .trim()
            .replace("  ", " ")
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let conts: Vec<u32> = card
            .1
            .trim()
            .replace("  ", " ")
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        for win_num in wins {
            if conts.contains(&win_num) {
                card_val += 1;
            }
        }
        cards.push(card_val);
        card_copies.push(1);
    }

    for i in 0..cards.len() {
        for j in 0..cards[i] {
            println!("{i}{j}");
            card_copies[i + j as usize + 1] += card_copies[i];
        }
    }

    card_copies.iter().sum()
}
