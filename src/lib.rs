#[macro_export]
macro_rules! include_day1_input {
    ($path:expr) => {
        include_str!($path)
            .split("\n\n")
            .map(|chunk| {
                chunk
                    .trim()
                    .split("\n")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect()
    };
}

#[macro_export]
macro_rules! include_day2_input {
    ($path:expr) => {
        include_str!($path)
            .lines()
            .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
            .collect()
    };
}

#[macro_export]
macro_rules! include_day3_input {
    ($path:expr) => {
        include_str!($path)
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    };
}

#[macro_export]
macro_rules! include_day4_input {
    ($path:expr) => {
        include_str!($path)
            .lines()
            .map(|line| {
                let (first, second) = line.split_once(',').unwrap();
                let (a, b) = first.split_once('-').unwrap();
                let (c, d) = second.split_once('-').unwrap();
                (
                    (a.parse().unwrap(), b.parse().unwrap()),
                    (c.parse().unwrap(), d.parse().unwrap()),
                )
            })
            .collect()
    };
}
