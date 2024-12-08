use regex::Regex;

pub struct Mul(u32, u32);

pub fn part_1(instructions: &Vec<Mul>) -> u32 {
    instructions.iter().map(|Mul(x, y)| x * y).sum()
}

pub fn part_2(instructions: &Vec<Mul>) -> u32 {
    1
}

pub fn parse_input(input: &str) -> Vec<Mul> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let (_, [x, y]) = c.extract();
            Mul(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(&parse_input(EXAMPLE_INPUT)), 161);
    }
}
