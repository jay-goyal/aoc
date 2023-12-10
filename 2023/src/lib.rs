use std::str::FromStr;

pub fn read_one_per_line<T>(path: &str) -> Option<Vec<T>>
where
    T: FromStr,
{
    Some(
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .filter_map(|line| line.parse::<T>().ok())
            .collect(),
    )
}
