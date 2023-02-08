use lazy_static::lazy_static;
use std::{collections::HashMap, hash::Hash, io};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

lazy_static! {
    static ref SIGN_MAP: HashMap<char, Sign> = HashMap::from([
        ('A', Sign::Rock),
        ('B', Sign::Paper),
        ('C', Sign::Scissors),
        ('X', Sign::Rock),
        ('Y', Sign::Paper),
        ('Z', Sign::Scissors),
    ]);
}

lazy_static! {
    static ref SIGN_TO_SCORE: HashMap<Sign, usize> =
        HashMap::from([(Sign::Rock, 1), (Sign::Paper, 2), (Sign::Scissors, 3),]);
}

lazy_static! {
    static ref WINS_AGAINST: HashMap<Sign, Sign> = HashMap::from([
        (Sign::Rock, Sign::Scissors),
        (Sign::Paper, Sign::Rock),
        (Sign::Scissors, Sign::Paper),
    ]);
}

fn main() {
    let lines :Vec<String>= std::fs::read_to_string("input.txt")
        .expect("could not open file")
        .split("\n").map(|s| s.to_owned()).collect();
    let score :usize= lines.iter().filter(|line| !line.is_empty()).map(|line| line_to_tuple(line)).map(tuple_to_score).sum();
    println!("the score is: {score}")
}


fn line_to_tuple(line: &str) -> (Sign, Sign) {
    println!("{line}");
    let chars = line.chars().collect::<Vec<_>>();
    (
        *SIGN_MAP.get(&chars[0]).expect(format!("failed on char '{}'", &chars[0]).as_str()),
        *SIGN_MAP.get(&chars[2]).expect(format!("failed on char '{}'", &chars[0]).as_str()),
    )
}

fn tuple_to_score(signs: (Sign, Sign)) -> usize {
    let sign_score = SIGN_TO_SCORE.get(&signs.1).unwrap();
    let match_score: usize = match WINS_AGAINST.get(&signs.1).unwrap() == &signs.0 {
        true => 6,
        _ => {
            if signs.0 == signs.1 {
                3
            } else {
                0
            }
        }
    };
    return sign_score + match_score;
}
