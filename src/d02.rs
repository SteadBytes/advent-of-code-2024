type Report = Vec<u32>;

pub fn part_1(reports: &Vec<Report>) -> usize {
    reports.iter().filter(|r| is_safe(r)).count()
}

// FIXME: Avoid all this copying
pub fn part_2(reports: &Vec<Report>) -> usize {
    reports
        .iter()
        .filter(|r| {
            if is_safe(r) {
                return true;
            };
            for i in 0..r.len() {
                let mut r2 = r.to_vec();
                r2.remove(i);
                if is_safe(&r2) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn is_safe(report: &Report) -> bool {
    let mut last = report[0];
    let direction = report[0] < report[1];
    report.iter().skip(1).all(|level| {
        let diff = level.abs_diff(last);
        if !(1..=3).contains(&diff) {
            return false;
        }
        if (last < *level) != direction {
            return false;
        }
        last = *level;
        true
    })
}

pub fn parse_input(input: &str) -> Vec<Report> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(&parse_input(EXAMPLE_INPUT)), 2);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(&parse_input(EXAMPLE_INPUT)), 4);
    }
}
