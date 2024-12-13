use regex::Regex;

pub enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

use Instruction::*;

pub fn part_1(instructions: &Vec<Instruction>) -> u32 {
    instructions
        .iter()
        .filter_map(|i| match i {
            Mul(x, y) => Some(x * y),
            _ => None,
        })
        .sum()
}

pub fn part_2(instructions: &Vec<Instruction>) -> u32 {
    1
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
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

    #[test]
    fn part_1_example() {
        assert_eq!(
            part_1(&parse_input(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )),
            161
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            part_2(&parse_input(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            48
        );
    }
}
