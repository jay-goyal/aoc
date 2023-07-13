use std::cmp::Reverse;

fn main() {
    let lines = aoc::read_one_per_line::<String>("./data/day1.txt").unwrap();
    let mut sum: usize = 0;
    let mut max_sum: [usize; 3] = [0; 3];
    for line in lines {
        if line.is_empty() {
            if sum > max_sum[2] {
                max_sum[2] = sum;
            }
            max_sum.sort_by_key(|x| Reverse(*x));
            sum = 0;
            continue;
        }
        sum += line.parse::<usize>().unwrap();
    }
    let top_three: usize = max_sum.iter().sum();
    println!("{top_three}");
}
