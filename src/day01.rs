use std::collections::HashMap;
use crate::input::daily_input;

fn calibration(input: &String) -> u32 {
    let digits = input.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
    return digits[0] * 10 + digits[digits.len() - 1];
}

fn decode_spellings(input: &String) -> String {
    let digits_spelled_out = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("nine", "n9e"),
        ("eight", "e8t"),
    ]);
    digits_spelled_out
        .iter()
        .fold(input.to_string(), |result, (spelling, code)| { result.replace(spelling, code) })
}

fn total_calibration(input: Vec<String>, account_for_spelled_outs: bool) -> u32 {
    input.iter()
        .map(|input| match account_for_spelled_outs { false => calibration(input), true => calibration(&decode_spellings(input)) })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let total = total_calibration(daily_input(1), false);
        assert_eq!(total, 55447);
    }

    #[test]
    fn part_two() {
        let total = total_calibration(daily_input(1), true);
        assert_eq!(total, 54706);
    }
}
