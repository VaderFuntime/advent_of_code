use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const FILENAME: &str = "elf_carry.txt";

fn main() {
    let mut amounts : Vec<i32> = fs::read_to_string(FILENAME)
        .expect("error reading file")
        .split("\n\n")
        .map(|rows| {
            rows.split('\n')
                .map(|row| row.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        }).collect();
    
    amounts.sort();
    amounts.reverse();
    println!("top 3 are: {:?}, sum = {}", &amounts[..3], amounts[..3].iter().sum::<i32>())
}
