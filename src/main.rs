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
}
