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
    let lines: Vec<String> = std::fs::read_to_string("input.txt")
        .expect("could not open file")
        .split("\n")
        .map(|s| s.to_owned())
        .collect();
    let score: usize = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line_to_tuple_p2(line))
        .map(tuple_to_score_p2)
        .sum();
    println!("the score is: {score}")
}




fn line_to_tuple(line: &str) -> (Sign, Sign) {
    println!("{line}");
    let chars = line.chars().collect::<Vec<_>>();
    (
        *SIGN_MAP
            .get(&chars[0])
            .expect(format!("failed on char '{}'", &chars[0]).as_str()),
        *SIGN_MAP
            .get(&chars[2])
            .expect(format!("failed on char '{}'", &chars[0]).as_str()),
    )
}

fn get_winner(sign: &Sign) -> Sign {
    match sign {
        Sign::Rock => Sign::Paper,
        Sign::Paper => Sign::Scissors,
        Sign::Scissors => Sign::Rock,
    }
}

fn get_loser(sign: &Sign) -> Sign {
    match sign {
        Sign::Paper => Sign::Rock,
        Sign::Scissors => Sign::Paper,
        Sign::Rock => Sign::Scissors,
    }
}

fn get_my_sign(signs: (Sign, Res)) -> Sign {
    let (their_sign, res) = signs;
    match res {
        Res::Draw => their_sign.clone(),
        Res::Win => get_winner(&their_sign),
        Res::Lose => get_loser(&their_sign),
    }
}

fn res_to_score(res : &Res) -> usize {
    match res {
        Res::Lose => 0,
        Res:: Draw => 3,
        Res::Win => 6,
    }
}

fn tuple_to_score_p2(signs: (Sign, Res)) -> usize {
    let (their_sign, res) = signs;
    let my_sign = get_my_sign(signs);
    SIGN_TO_SCORE.get(&my_sign).unwrap() + res_to_score(&res)
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Res {
    Win,
    Lose,
    Draw,
}

fn char_to_res(c: char) -> Option<Res> {
    match c {
        'X' => Some(Res::Lose),
        'Y' => Some(Res::Draw),
        'Z' => Some(Res::Win),
        _ => None,
    }
}

fn line_to_tuple_p2(line: &str) -> (Sign, Res) {
    // println!("{line}");
    let chars = line.chars().collect::<Vec<_>>();
    (
        *SIGN_MAP
            .get(&chars[0])
            .expect(format!("failed on char '{}'", &chars[0]).as_str()),
        char_to_res(chars[2]).unwrap(),
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
