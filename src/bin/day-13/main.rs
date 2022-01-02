use std::{
    collections::{HashMap, VecDeque},
    ops::Deref,
    vec,
};

use aoc_2015::{lines_for_day, lines_for_day_test};

fn main() {
    let lines = lines_for_day_test("day-13");
    let happiness = parse(lines);
    let all_arrangements = combos(happiness.keys().collect::<Vec<_>>());
    println!("{:#?}", all_arrangements.len());
}

fn combos(people: Vec<&Person>) -> VecDeque<VecDeque<Person>> {
    if people.len() == 1 {
        vec![vec![people[0].clone()].into()].into()
    } else {
        let mut vecs: VecDeque<VecDeque<Person>> = VecDeque::new();
        for person in people.clone() {
            let mut without_person = combos(
                people
                    .clone()
                    .into_iter()
                    .filter(|o| person != *o)
                    .collect::<Vec<_>>(),
            );
            without_person
                .iter_mut()
                .for_each(|v| v.push_front(person.clone()));

            without_person
                .into_iter()
                .for_each(|v| vecs.push_front(v.clone()));
        }
        vecs
    }
}

fn parse(lines: Vec<String>) -> HappinessMap {
    let mut map: HappinessMap = HashMap::new();

    for line in lines {
        let (name, rest) = line.split_once(" would ").expect("could not split");
        let (num, rest) = if rest.starts_with("gain") {
            let (num, rest) = rest
                .trim_start_matches("gain ")
                .split_once(" ")
                .expect("could not split");
            (num.parse::<i32>().expect("could not parse"), rest)
        } else {
            let (num, rest) = rest
                .trim_start_matches("lose ")
                .split_once(" ")
                .expect("could not split");
            (-1 * num.parse::<i32>().expect("could not parse"), rest)
        };

        let other = rest
            .to_string()
            .split_off(35)
            .trim_end_matches('.')
            .to_string();

        map.entry(name.into())
            .or_default()
            .entry(other.into())
            .or_insert(num);
    }

    map
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Person(String);

impl Deref for Person {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Person {
    fn from(string: &str) -> Self {
        Self(string.to_string())
    }
}

impl From<String> for Person {
    fn from(string: String) -> Self {
        Self(string)
    }
}

type HappinessMap = HashMap<Person, HashMap<Person, i32>>;
