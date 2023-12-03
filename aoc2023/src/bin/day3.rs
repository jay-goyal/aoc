use std::collections::{HashMap, HashSet};

fn main() {
    let lines = aoc2023::read_one_per_line::<String>("./data/day3.txt").unwrap();
    let chars: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut num = 0;
    let mut num_adj = false;
    let mut stars: HashMap<(usize, usize), (u32, u64)> = HashMap::new();
    for (i, iv) in chars.iter().enumerate() {
        let mut curr_stars: HashSet<(usize, usize)> = HashSet::new();
        for (j, jc) in iv.iter().enumerate() {
            if jc.is_digit(10) {
                num = num * 10 + jc.to_digit(10).unwrap();
                let ret_val = is_adjacent(i as i32, j as i32, &chars);
                if !num_adj {
                    num_adj = ret_val.0;
                }
                curr_stars.extend(ret_val.1);
            } else {
                if num_adj {
                    sum1 += num;
                    for star in &curr_stars {
                        match stars.get(star) {
                            Some(x) => {
                                if x.0 < 2 {
                                    stars.insert(star.clone(), (x.0 + 1, x.1 * num as u64))
                                } else {
                                    stars.insert(star.clone(), (x.0 + 1, x.1))
                                }
                            }
                            None => stars.insert(star.clone(), (1, num as u64)),
                        };
                    }
                }
                curr_stars.clear();
                num = 0;
                num_adj = false;
            }
        }
        if num_adj {
            sum1 += num;
        }
        for star in &curr_stars {
            match stars.get(star) {
                Some(x) => {
                    if x.0 < 2 {
                        stars.insert(star.clone(), (x.0 + 1, x.1 * num as u64))
                    } else {
                        stars.insert(star.clone(), (x.0 + 1, x.1))
                    }
                }
                None => stars.insert(star.clone(), (1, num as u64)),
            };
        }
        num = 0;
        num_adj = false;
    }

    dbg!(&stars);

    for (_, star) in stars {
        if star.0 == 2 {
            sum2 += star.1;
        }
    }

    println!("PART 1 ANS: {sum1}");
    println!("PART 2 ANS: {sum2}");
}

fn is_adjacent(r: i32, c: i32, vec: &Vec<Vec<char>>) -> (bool, HashSet<(usize, usize)>) {
    let checks = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    let mut r_par = false;
    let mut r_set = HashSet::new();

    for check in checks {
        let new_row = r + check[0];
        let new_col = c + check[1];
        if new_row >= 0
            && new_col >= 0
            && new_row < vec.len() as i32
            && new_col < vec[0].len() as i32
        {
            let c = vec[new_row as usize][new_col as usize];
            if !c.is_digit(10) && c != '.' {
                r_par = true;
                if c == '*' {
                    r_set.insert((new_row as usize, new_col as usize));
                }
            }
        }
    }

    (r_par, r_set)
}
