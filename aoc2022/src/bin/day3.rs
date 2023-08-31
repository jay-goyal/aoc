use std::collections::HashSet;

fn main() {
    let lines = aoc::read_one_per_line::<String>("./data/day3.txt").unwrap();
    let mut sum: u32 = 0;
    for line in lines {
        let l = line.len();
        let mut hs: HashSet<char> = HashSet::new();

        for i in l / 2..l {
            let ch = line.chars().nth(i).unwrap();
            if line
                .chars()
                .position(|x| x == ch)
                .unwrap()
                < l / 2
                && !hs.contains(&ch)
            {
                if ch as u8 >= 97 {
                    sum += (ch as u8 - 96) as u32;
                } else {
                    sum += (ch as u8 - 64 + 26) as u32;
                }
                hs.insert(ch);
            }
        }
    }
    println!("{sum}");
    println!("*********************************************");

    let mut c = 0;
    let mut v = vec![];
    let lines = aoc::read_one_per_line::<String>("./data/day3.txt").unwrap();
    sum = 0;
    'outer: for line in lines {
        if c < 2 {
            v.push(line);
            c += 1;
        } else {
            c = 0;
            'checker: for ch in line.chars() {
                for i in 0..2 {
                    match v[i].chars().position(|x| x == ch) {
                        Some(_) => continue,
                        None => continue 'checker,
                    }
                }
                if ch as u8 >= 97 {
                    sum += (ch as u8 - 96) as u32;
                } else {
                    sum += (ch as u8 - 64 + 26) as u32;
                }
                println!("ADDING: {ch}");
                v.clear();
                continue 'outer;
            }
        }
    }

    println!("{sum}");
}
