fn main() {
    let lines = aoc2023::read_one_per_line::<String>("./data/day2.txt").unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in &lines {
        let mut cubes: [u64; 3] = [0, 0, 0];
        let (game, cube_sets) = line.split_once(": ").unwrap();
        for cube_set in cube_sets.split("; ") {
            for cube in cube_set.split(", ") {
                let (num, col) = cube.split_once(" ").unwrap();
                match col.trim() {
                    "red" => cubes[0] = cubes[0].max(num.parse::<u64>().unwrap()),
                    "green" => cubes[1] = cubes[1].max(num.parse::<u64>().unwrap()),
                    "blue" => cubes[2] = cubes[2].max(num.parse::<u64>().unwrap()),
                    _ => panic!("Invalid color"),
                }
            }
        }
        if cubes[0] <= 12 && cubes[1] <= 13 && cubes[2] <= 14 {
            sum1 += game.split_once(" ").unwrap().1.parse::<u64>().unwrap();
        }

        sum2 += cubes[0] * cubes[1] * cubes[2];
        // only 12 red cubes, 13 green cubes, and 14 blue cubes
    }

    println!("PART 1 ANS: {sum1}");
    println!("PART 2 ANS: {sum2}");
}
