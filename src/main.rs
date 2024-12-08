use aoc2024::*;
use std::cmp::min;
use std::time::Duration;
use std::time::Instant;

fn pretty_print(line: &str, output: Option<&str>, duration: Duration) {
    const DISPLAY_WIDTH: usize = 40;

    let duration = format!("({:.2?})", duration);
    print!("{} {}", line, duration);

    match output {
        Some(output) => {
            let width = "  - ".len() + line.chars().count() + 1 + duration.chars().count();
            let dots = DISPLAY_WIDTH - min(DISPLAY_WIDTH - 5, width) - 2;
            print!(" {}", ".".repeat(dots));

            if output.contains('\n') {
                println!();

                for line in output.trim_matches('\n').lines() {
                    println!("    {}", line);
                }
            } else {
                println!(" {}", output);
            }
        }
        None => println!(),
    }
}

// Time the given function, returning its result and the elapsed time
fn time<T>(func: &dyn Fn() -> T) -> (Duration, T) {
    let start = Instant::now();
    let result = func();

    (start.elapsed(), result)
}

// Adapted from https://gitlab.com/mbryant/aoc-2021/-/blob/main/src/main.rs?ref_type=heads
#[macro_export]
macro_rules! main {
    (
        implemented_days: [$($day:ident),+ $(,)?]
    ) => {
        const DAYS: &[&str] = &[$(stringify!($day)),*];
        const INPUTS : &[&str] = &[$(include_str!(concat!("../inputs/", stringify!($day), ".txt"))),*];

        fn main() {
            let (elapsed, _) = time(&|| {
                let days: Vec<u32> = match std::env::args().nth(1) {
                    Some(day) => {
                        let d = day.parse().expect("invalid day number");
                        assert!(DAYS.contains(&format!("d{:02}", day).as_ref()), "Requested an unimplemented day");
                        vec![d]
                    },
                    None => DAYS.iter().map(|s| s[1..].parse().expect("Unexpected day module name")).collect()
                };

                for day in days.into_iter() {
                    let module_name = format!("d{:02}", day);
                    match module_name.as_ref() {
                        $(stringify!($day) => {
                            let data = INPUTS[day as usize - 1];

                            let (parse_input_elapsed, input) = time(&|| $day::parse_input(&data));
                            let (p1_elapsed, p1_result) = time(&|| $day::part_1(&input));
                            let (p2_elapsed, p2_result) = time(&|| $day::part_2(&input));

                            let duration = format!("({:.2?})", parse_input_elapsed + p1_elapsed + p2_elapsed);
                            println!("{} {}", format!("Day {}", day), duration);
                            pretty_print(" · Parse input", None, parse_input_elapsed);
                            pretty_print(" · Part 1", Some(&format!("{}", p1_result)), p1_elapsed);
                            pretty_print(" · Part 2", Some(&format!("{}", p2_result)), p2_elapsed);
                            println!()
                        },)+
                        _ => unreachable!() // All the days should've been hit by the match
                    }
                }
            });
            println!("{} {}", "Overall runtime", format!("({:.2?})", elapsed));
        }
    };
}

main! {
    implemented_days: [
        d01,
        d02,
        d03,
    ]
}
