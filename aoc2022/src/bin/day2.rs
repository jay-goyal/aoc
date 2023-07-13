use std::collections::HashMap;

fn main() {
    let winner: HashMap<&str, &str> = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);
    let equals: HashMap<&str, &str> = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
    let pts: HashMap<&str, usize> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let mut score_wrong: usize = 0;
    let mut score_right: usize = 0;
    let lines = aoc::read_one_per_line::<String>("./data/day2.txt").unwrap();
    for line in lines {
        let v: Vec<&str> = line.split(|c| c == ' ').collect();
        if winner[v[0]] == v[1] {
            score_wrong += pts[v[1]];
        } else if equals[v[0]] == v[1] {
            score_wrong += pts[v[1]] + 3;
        } else {
            score_wrong += pts[v[1]] + 6;
        }
        if v[1] == "Z" {
            score_right += 12 - pts[equals[v[0]]] - pts[winner[v[0]]];
        } else if v[1] == "Y" {
            score_right += 3 + pts[equals[v[0]]];
        } else {
            score_right += pts[winner[v[0]]];
        }
    }
    println!("WRONG: {score_wrong}");
    println!("RIGHT: {score_right}");
}
