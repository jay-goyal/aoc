use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
};

fn main() {
    let lines = aoc2023::read_one_per_line::<String>("./data/day7.txt").unwrap();
    let ans1 = part1(&lines);
    let ans2 = part2(&lines);
    println!("PART 1 ANS: {ans1}");
    println!("PART 2 ANS: {ans2}");
}

fn part1(lines: &Vec<String>) -> u64 {
    let mut ans = 0;
    let ranks = lines
        .iter()
        .map(|x| {
            let tup = x.split_once(" ").unwrap();
            (tup.0.to_string(), tup.1.parse::<u64>().unwrap())
        })
        .collect::<HashMap<String, u64>>();
    let mut hands = ranks.keys().collect::<Vec<&String>>();

    hands.sort_unstable_by(|&a, &b| comp_hands(a, b, "one"));

    for (i, &hand) in hands.iter().enumerate() {
        ans += ranks.get(hand).unwrap() * (i as u64 + 1);
    }

    return ans;
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut ans = 0;
    let ranks = lines
        .iter()
        .map(|x| {
            let tup = x.split_once(" ").unwrap();
            (tup.0.to_string(), tup.1.parse::<u64>().unwrap())
        })
        .collect::<HashMap<String, u64>>();
    let mut hands = ranks.keys().collect::<Vec<&String>>();

    hands.sort_unstable_by(|&a, &b| comp_hands(a, b, "two"));

    for (i, &hand) in hands.iter().enumerate() {
        ans += ranks.get(hand).unwrap() * (i as u64 + 1);
    }

    return ans;
}

fn get_hand_val(s: &str, part: &str) -> u64 {
    let mut occ: HashMap<char, u8> = HashMap::new();
    for c in s.chars() {
        match occ.get(&c) {
            Some(&x) => occ.insert(c, x + 1),
            None => occ.insert(c, 1),
        };
    }

    if part.to_lowercase() == "two" {
        let val = occ.get(&'J');
        match val {
            Some(&x) => {
                let mut temp = occ
                    .iter()
                    .map(|tup| (tup.0.to_owned(), tup.1.to_owned()))
                    .collect::<Vec<_>>();

                temp.sort_by_key(|k| Reverse(k.1));

                if temp[0].0 == 'J' {
                    if temp.len() > 1 {
                        occ.insert(temp[1].0, temp[1].1 + x);
                        occ.remove(&'J');
                    }
                } else {
                    occ.insert(temp[0].0, temp[0].1 + x);
                    occ.remove(&'J');
                }
            }
            None => {}
        }
    }

    let mut occ_vals = occ.into_values().collect::<Vec<u8>>();
    occ_vals.sort_by_key(|w| Reverse(*w));

    let base_val = match occ_vals[..] {
        [5] => 700,
        [4, 1] => 600,
        [3, 2] => 500,
        [3, 1, 1] => 400,
        [2, 2, 1] => 300,
        [2, 1, 1, 1] => 200,
        [1, 1, 1, 1, 1] => 100,
        _ => {
            println!("{}: {:?}", s, occ_vals);
            panic!("Invalid Input")
        }
    };

    return base_val;
}

fn comp_hands(a: &String, b: &String, part: &str) -> Ordering {
    let mut a_val = get_hand_val(a, part);
    let mut b_val = get_hand_val(b, part);

    let mut a_char = a.chars();
    let mut b_char = b.chars();

    while a_val == b_val {
        a_val += get_char_val(a_char.next().unwrap(), part);
        b_val += get_char_val(b_char.next().unwrap(), part);
    }

    if a_val > b_val {
        return Ordering::Greater;
    } else if a_val < b_val {
        return Ordering::Less;
    }

    Ordering::Equal
}

fn get_char_val(c: char, part: &str) -> u64 {
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => {
            if part.to_lowercase() == "one" {
                10
            } else {
                0
            }
        }
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0,
    }
}
