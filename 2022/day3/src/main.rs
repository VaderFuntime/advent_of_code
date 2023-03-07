use std::collections::{hash_map::RandomState, HashSet};

fn main() {
    part2();
}

fn part2() {
    let lines: Vec<String> = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.to_owned())
        .collect();
    
    let mut sum = 0;
        for i in 0..(lines.len() / 3) {
        let s1 = &lines[i * 3];
        let s2 = &lines[i * 3 + 1];
        let s3 = &lines[i * 3 + 2];
        let letter = three_sacks_to_common_item(s1, s2, s3);
        sum += letter_to_priority(&letter);
    }
    println!("the sum is {sum}")
}

fn three_sacks_to_common_item(s1: &str, s2: &str, s3: &str) -> char {
    let first_items: HashSet<char, RandomState> = HashSet::from_iter(s1.chars());
    let second_items: HashSet<char, RandomState> = HashSet::from_iter(s2.chars());
    let third_items: HashSet<char, RandomState> = HashSet::from_iter(s3.chars());

    // Return the only char that is in all three sets
    first_items
        .intersection(&second_items)
        .cloned()
        .collect::<HashSet<char, RandomState>>()
        .intersection(&third_items)
        .cloned()
        .collect::<HashSet<char, RandomState>>()
        .iter()
        .next()
        .unwrap()
        .clone()
}

fn part1() {
    let sum = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(rucksack_to_common)
        .map(|letter| letter_to_priority(&letter))
        .sum::<usize>();
    println!("the sum is {sum}")
}

fn letter_to_priority(letter: &char) -> usize {
    match letter {
        'a'..='z' => (*letter as usize) - ('a' as usize) + 1,
        'A'..='Z' => (*letter as usize) - ('A' as usize) + 27,
        _ => panic!("invalid letter"),
    }
}

fn rucksack_to_common(rucksack: &str) -> char {
    let first_half: HashSet<char, RandomState> =
        HashSet::from_iter(rucksack[..(rucksack.len() / 2)].chars());
    let second_half: HashSet<char, RandomState> =
        HashSet::from_iter(rucksack[(rucksack.len() / 2)..].chars());

    first_half
        .intersection(&second_half)
        .next()
        .unwrap()
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_to_priority() {
        assert_eq!(letter_to_priority(&'a'), 1);
        assert_eq!(letter_to_priority(&'z'), 26);
        assert_eq!(letter_to_priority(&'A'), 27);
        assert_eq!(letter_to_priority(&'Z'), 52);
    }

    #[test]
    fn test_example_input() {
        let sum = std::fs::read_to_string("example_input.txt")
            .unwrap()
            .split("\n")
            .map(rucksack_to_common)
            .map(|letter| letter_to_priority(&letter))
            .sum::<usize>();
        assert_eq!(sum, 157)
    }
}
