fn main() {
    let input: Vec<Vec<u32>> = include_str!("../input/1a.txt")
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
        .collect();

    println!("Day 1a: {}", day1_a(&input));

    println!("Day 2b: {}", day1_b(input));
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_1a() {
        let input: Vec<Vec<u32>> = include_str!("../input/1a_test.txt")
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
            .collect();

        assert_eq!(day1_a(&input), 24000);
    }

    #[test]
    fn test_day_1b() {
        let input: Vec<Vec<u32>> = include_str!("../input/1a_test.txt")
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
            .collect();

        assert_eq!(day1_b(input), 45000);
    }
}
