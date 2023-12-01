fn main() {
    // PART 1
    let lines = aoc2023::read_one_per_line::<String>("./data/day1.txt").unwrap();
    let mut nums: Vec<u32> = vec![];

    for line in &lines {
        let mut first = None;
        let mut last = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                match first {
                    None => {
                        first = Some(c.to_digit(10).unwrap());
                        last = first.unwrap();
                    }
                    Some(_) => last = c.to_digit(10).unwrap(),
                }
            }
        }
        nums.push(first.unwrap_or(0) * 10 + last);
    }

    let mut sum: u32 = nums.iter().sum();
    println!("PART 1 ANS IS: {sum}");

    // PART 2
    nums.clear();

    for line in &lines {
        let l = line
            .to_string()
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");

        let mut first = None;
        let mut last = 0;

        for c in l.chars() {
            if c.is_digit(10) {
                match first {
                    None => {
                        first = Some(c.to_digit(10).unwrap());
                        last = first.unwrap();
                    }
                    Some(_) => last = c.to_digit(10).unwrap(),
                }
            }
        }
        nums.push(first.unwrap() * 10 + last);
    }
    sum = nums.iter().sum();
    println!("PART 2 ANS IS: {sum}");
}
