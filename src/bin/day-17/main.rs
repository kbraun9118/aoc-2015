#![feature(exclusive_range_pattern)]
use aoc_2015::lines_for_day_test;

fn main() {
    let mut lines = lines_for_day_test("day-17")
        .into_iter()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    lines.sort();

    println!("{:?}", sum_to(&lines, 25));
}

fn sum_to(rest: &[i32], sum: i32) -> i32 {
    dbg!(rest, sum);
    match (rest, sum) {
        (_, 0) => 1,
        ([rest], sum) if *rest == sum => 1,
        (_, i32::MIN..0) | ([], _) | ([_], _) => 0,
        (rest, sum) => (0..rest.len() - 1)
            .map(|i| sum_to(&rest[i + 1..], sum - rest[i]))
            .sum(),
    }
}
