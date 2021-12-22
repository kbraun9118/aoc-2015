use std::collections::HashMap;
use aoc_2015::lines_for_day;

fn main() {
    let lines = lines_for_day("day-05");

    let part_one = lines
        .iter()
        .map(part_one_is_nice)
        .filter(|p| *p)
        .count();

    let part_two = lines
        .iter()
        .map(part_two_is_nice)
        .filter(|p| *p)
        .count();

    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two);
    println!("{}", part_two_is_nice(&String::from("ieodomkazucvgmuy")))
}

fn part_one_is_nice(input: &String) -> bool {
    let char_map: HashMap<char, i32> = input
        .chars()
        .fold(HashMap::new(), |mut acc, next| {
            *acc.entry(next).or_default() += 1;
            acc
        });

    let has_double = input.chars()
        .zip(input.chars().skip(1))
        .any(|(l, r)| l == r);
    let two_vowels = char_map.get(&'a').unwrap_or(&0)
        + char_map.get(&'e').unwrap_or(&0)
        + char_map.get(&'i').unwrap_or(&0)
        + char_map.get(&'o').unwrap_or(&0)
        + char_map.get(&'u').unwrap_or(&0) > 2;
    let no_bad = !input.contains("ab") && !input.contains("cd") && !input.contains("pq") && !input.contains("xy");

    has_double && two_vowels && no_bad
}

fn part_two_is_nice(input: &String) -> bool {
    let repeats_with_middle = input.chars()
        .zip(input.chars().skip(1))
        .zip(input.chars().skip(2))
        .any(|((l, _), r)| l == r);

    let pair_repeats = input.chars()
        .zip(input.chars().skip(1))
        .any(|(l, r)| input.replacen(&format!("{}{}", l, r), "", 1).contains(&format!("{}{}", l, r)));

    if !repeats_with_middle {
        println!("No middle repeats: {}", input);
    }
    if !pair_repeats {
        println!("No pair repeats:   {}", input);
    }

    repeats_with_middle && pair_repeats
}
