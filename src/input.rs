use std::fs::read_to_string;

pub fn read_lines(filename: String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn daily_input(day: u32) -> Vec<String> {
    read_lines(format!("src/inputs/day{:0>2}.txt", day))
}

pub fn daily_example(day: u32) -> Vec<String> {
    read_lines(format!("src/examples/day{:0>2}.txt", day))
}

