#![feature(exclusive_range_pattern)]
use std::collections::HashSet;

use aoc_2015::{lines_for_day, lines_for_day_test};
use regex::Regex;

fn main() {
    let lines = lines_for_day("day-19");
    let (replacements, molecule) = lines.split_at(lines.len() - 2);

    let replacements = replacements
        .into_iter()
        .map(Replacement::from)
        .collect::<Vec<_>>();
    let regex = Regex::new(r"[A-Z][a-z]*").unwrap();
    let molecules = regex
        .captures_iter(molecule[1].as_str())
        .map(|c| c.get(0).unwrap().as_str().to_string())
        .collect::<Vec<_>>();

    let expanded = molecules
        .iter()
        .enumerate()
        .flat_map(|(i, m)| {
            let mut items = vec![];
            for r in replacements.iter().filter(|r| r.key == *m) {
                let mut new_molecule = molecules.clone();
                new_molecule[i] = r.value.clone();
                items.push(new_molecule.join(""))
            }
            items
        })
        .collect::<HashSet<_>>();

    println!("{:?}", expanded.len());
}

#[derive(Debug)]
struct Replacement {
    key: String,
    value: String,
}

impl From<&String> for Replacement {
    fn from(value: &String) -> Self {
        let (key, value) = value.split_once(" => ").unwrap();
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}
