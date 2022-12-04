macro_rules! include_day3_input {
    ($path:expr) => {
        include_str!($path)
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    };
}

macro_rules! include_day2_input {
    ($path:expr) => {
        include_str!($path)
            .lines()
            .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
            .collect()
    };
}

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

fn main() {
    let input: Vec<Vec<u32>> = include_day1_input!("../input/1a.txt");
    println!("Day 1a: {}", day1_a(&input));
    println!("Day 1b: {}", day1_b(input));

    let input: Vec<(char, char)> = include_day2_input!("../input/2a.txt");
    println!("Day 2a: {}", day2_a(&input));
    println!("Day 2b: {}", day2_b(input));

    let input: Vec<Vec<char>> = include_day3_input!("../input/3a.txt");
    println!("Day 3a: {}", day3_a(&input));
    println!("Day 3b: {}", day3_b(input));
}

pub fn day1_a(input: &Vec<Vec<u32>>) -> u32 {
    let summed: Vec<u32> = input
        .iter()
        .map(|list| list.iter().fold(0, |acc, x| acc + x))
        .collect();
    *summed.iter().max().unwrap()
}

pub fn day1_b(input: Vec<Vec<u32>>) -> u32 {
    let mut summed: Vec<u32> = input
        .iter()
        .map(|list| list.iter().fold(0, |acc, x| acc + x))
        .collect();
    summed.sort();
    let last_three = summed.get(summed.len() - 3..).unwrap();
    last_three.iter().sum()
}

pub fn day2_a(input: &Vec<(char, char)>) -> u32 {
    input.iter().fold(0, |acc, p| match (p.0, p.1) {
        ('A', 'X') => acc + 4,
        ('A', 'Y') => acc + 8,
        ('A', 'Z') => acc + 3,
        ('B', 'X') => acc + 1,
        ('B', 'Y') => acc + 5,
        ('B', 'Z') => acc + 9,
        ('C', 'X') => acc + 7,
        ('C', 'Y') => acc + 2,
        ('C', 'Z') => acc + 6,
        _ => acc,
    })
}

pub fn day2_b(input: Vec<(char, char)>) -> u32 {
    input.iter().fold(0, |acc, p| match (p.0, p.1) {
        ('A', 'X') => acc + 3,
        ('A', 'Y') => acc + 4,
        ('A', 'Z') => acc + 8,
        ('B', 'X') => acc + 1,
        ('B', 'Y') => acc + 5,
        ('B', 'Z') => acc + 9,
        ('C', 'X') => acc + 2,
        ('C', 'Y') => acc + 6,
        ('C', 'Z') => acc + 7,
        _ => acc,
    })
}

pub fn day3_a(input: &Vec<Vec<char>>) -> u32 {
    input
        .clone()
        .iter_mut()
        .map(|first: &mut Vec<char>| {
            let second = first.split_off(first.len() / 2);
            let dupe = *first.iter().filter(|x| second.contains(x)).next().unwrap();
            if dupe >= 'a' && dupe <= 'z' {
                return (dupe as u32) - 96;
            } else {
                return (dupe as u32) - 38;
            }
        })
        .sum()
}

pub fn day3_b(input: Vec<Vec<char>>) -> u32 {
    input
        .clone()
        .windows(3)
        .step_by(3)
        .map(|window| {
            let dupe = *window[0]
                .iter()
                .filter(|x| window[1].contains(x) && window[2].contains(x))
                .next()
                .unwrap();
            if dupe >= 'a' && dupe <= 'z' {
                return (dupe as u32) - 96;
            } else {
                return (dupe as u32) - 38;
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_1a() {
        let input: Vec<Vec<u32>> = include_day1_input!("../input/1a_test.txt");
        assert_eq!(day1_a(&input), 24000);
    }

    #[test]
    fn test_day_1b() {
        let input: Vec<Vec<u32>> = include_day1_input!("../input/1a_test.txt");
        assert_eq!(day1_b(input), 45000);
    }

    #[test]
    fn test_day_2a() {
        let input: Vec<(char, char)> = include_day2_input!("../input/2a_test.txt");
        assert_eq!(day2_a(&input), 15);
    }

    #[test]
    fn test_day_2b() {
        let input: Vec<(char, char)> = include_day2_input!("../input/2a_test.txt");
        assert_eq!(day2_b(input), 12);
    }

    #[test]
    fn test_day_3a() {
        let input: Vec<Vec<char>> = include_day3_input!("../input/3a_test.txt");
        assert_eq!(day3_a(&input), 157);
    }

    #[test]
    fn test_day_3b() {
        let input: Vec<Vec<char>> = include_day3_input!("../input/3a_test.txt");
        assert_eq!(day3_b(input), 70);
    }
}
