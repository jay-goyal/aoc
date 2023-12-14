fn main() {
    let lines = aoc2023::read_one_per_line::<String>("./data/day13.txt").unwrap();
    let parsed_lines = convert_to_req_format(&lines);
    let ans1 = solve(&parsed_lines, true);
    let ans2 = solve(&parsed_lines, false);
    println!("PART 1 ANS: {ans1}");
    println!("PART 2 ANS: {ans2}");
}
fn solve(lines: &Vec<Vec<Vec<char>>>, sf: bool) -> u64 {
    let mut ans = 0;

    'main: for grp in lines {
        let grp_len = grp.len();
        let grp_line_len = grp[0].len();

        for row in 0..(grp_len - 1) {
            let mut flag = sf;
            let mut smudge_flag = sf;
            let diff = usize::min(row + 1, grp_len - row - 1);
            'diff: for d in 0..diff {
                for col in 0..grp_line_len {
                    if grp[row - d][col] != grp[row + d + 1][col] {
                        if smudge_flag {
                            flag = false;
                            break 'diff;
                        } else {
                            smudge_flag = true;
                            flag = true;
                        }
                    }
                }
            }
            if flag {
                ans += 100 * (row as u64 + 1);
                continue 'main;
            }
        }

        for col in 0..(grp_line_len - 1) {
            let mut flag = sf;
            let mut smudge_flag = sf;
            let diff = usize::min(col + 1, grp_line_len - col - 1);
            'diff: for d in 0..diff {
                for row in 0..grp_len {
                    if grp[row][col - d] != grp[row][col + d + 1] {
                        if smudge_flag {
                            flag = false;
                            break 'diff;
                        } else {
                            smudge_flag = true;
                            flag = true;
                        }
                    }
                }
            }
            if flag {
                ans += col as u64 + 1;
                continue 'main;
            }
        }
    }

    ans
}

fn convert_to_req_format(lines: &Vec<String>) -> Vec<Vec<Vec<char>>> {
    let mut res = Vec::new();
    let mut res_val = Vec::new();
    for line in lines {
        if line.is_empty() {
            res.push(res_val);
            res_val = Vec::new();
        } else {
            res_val.push(line.chars().collect::<Vec<_>>());
        }
    }
    res.push(res_val.clone());

    res
}
