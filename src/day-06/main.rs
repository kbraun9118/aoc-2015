use aoc_2015::lines_for_day;

fn main() {
    // let commands = lines_for_day("day-06")
    //     .into_iter()
    //     .map(|s| Command::from(s))
    //     .collect::<Vec<_>>();
    //
    // println!("{:?}", commands);
    println!("{:?}", Square::new(0, 0, 10, 10)
        .intersection(Square::new(10, 10, 10, 10)));
}

#[derive(Debug, Copy, Clone)]
struct Square {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
}

impl Square {
    fn new(start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> Self {
        Square {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }

    fn intersection(self, other: Self) -> Option<Self> {
        if self.end_x <= other.start_x && self.end_y <= other.start_y
            || other.end_x <= self.start_x && other.end_y <= self.start_y {
            None
        } else {
            let start_x = self.start_x.max(other.start_x);
            let end_x = self.end_x.min(other.end_x);
            let start_y = self.start_y.max(other.start_y);
            let end_y = self.end_y.min(other.end_y);
            Some(Square {
                start_x,
                end_x,
                start_y,
                end_y,
            })
        }
    }
}

impl From<Vec<(&str, &str)>> for Square {
    fn from(vec: Vec<(&str, &str)>) -> Self {
        Square {
            start_x: vec[0].0.parse().expect("could not parse"),
            start_y: vec[0].1.parse().expect("could not parse"),
            end_x: vec[1].0.parse().expect("could not parse"),
            end_y: vec[1].1.parse().expect("could not parse"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Command {
    On(Square),
    Off(Square),
    Toggle(Square),
}

impl From<String> for Command {
    fn from(string: String) -> Self {
        if string.starts_with("turn on") {
            let rest = string[8..].split(" through ")
                .map(|s| s.split_once(',').expect("could not split"))
                .collect::<Vec<_>>();
            Self::On(Square::from(rest))
        } else if string.starts_with("turn off") {
            let rest = string[9..].split(" through ")
                .map(|s| s.split_once(',').expect("could not split"))
                .collect::<Vec<_>>();
            Self::Off(Square::from(rest))
        } else {
            let rest = string[7..].split(" through ")
                .map(|s| s.split_once(',').expect("could not split"))
                .collect::<Vec<_>>();
            Self::Toggle(Square::from(rest))
        }
    }
}
