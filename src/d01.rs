use std::collections::HashMap;

pub fn part_1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut left = left.clone();
    let mut right = right.clone();
    left.sort();
    right.sort();

    left.iter().zip(right).map(|(x, y)| x.abs_diff(y)).sum()
}

pub fn part_2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let freq_right: HashMap<u32, u32> = right.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_default() += 1;
        acc
    });

    left.iter()
        .map(|x| freq_right.get(x).unwrap_or(&0) * x)
        .sum()
}

pub fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let mut parts = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
        left.push(parts.next().unwrap());
        right.push(parts.next().unwrap());
        assert!(parts.next().is_none());
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(&parse_input(EXAMPLE_INPUT)), 11);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(&parse_input(EXAMPLE_INPUT)), 31);
    }
}
