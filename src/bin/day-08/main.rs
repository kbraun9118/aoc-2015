use aoc_2015::lines_for_day;
use regex::Regex;

fn main() {
    let lines = lines_for_day("day-08");

    let part_one: usize = lines
        .iter()
        .zip(lines.iter().map(real_string))
        .map(|(left, right)| left.len() - right.len())
        .sum();

    let part_two: usize = lines
        .iter()
        .zip(lines.iter().map(expanded_string))
        .map(|(left, right)| 2 + right.len() - left.len())
        .sum();

    println!("Part One: {}", part_one);
    println!("Part One: {}", part_two);
}

fn real_string(string: &String) -> String {
    string
        .trim_end_matches("\"")
        .trim_start_matches("\"")
        .replace(r"\\", r"\")
        .replace(r#"\""#, r#"""#)
        .replace(&Regex::new(r"(\\x([0-9]|[a-f]){2})").unwrap(), "A")
}

fn expanded_string(string: &String) -> String {
    string.replace(r"\", r"\\").replace("\"", r#"\""#)
}
