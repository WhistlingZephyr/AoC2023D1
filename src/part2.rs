use lazy_static::lazy_static;
use regex::Regex;

#[inline(always)]
fn get_digit(text: &str) -> Option<usize> {
    Some(match text {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => return None,
    })
}
lazy_static! {
    static ref START_REGEX: Regex = Regex::new(
        &(0..80)
            .map(|i| format!(r"^.{{{i}}}(\d|one|two|three|four|five|six|seven|eight|nine)"))
            .collect::<Vec<_>>()
            .join("|")
    )
    .unwrap();
    static ref END_REGEX: Regex = Regex::new(&format!(
        r".*(?:{})",
        (0..80)
            .map(|i| format!(r"(\d|one|two|three|four|five|six|seven|eight|nine).{{{i}}}$"))
            .collect::<Vec<_>>()
            .join("|")
    ))
    .unwrap();
}

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let start = get_digit(START_REGEX.captures(line).unwrap().extract::<1>().1[0]).unwrap();
            let end = get_digit(END_REGEX.captures(line).unwrap().extract::<1>().1[0]).unwrap();
            format!("{start}{end}").parse::<usize>().unwrap()
        })
        .sum()
}
