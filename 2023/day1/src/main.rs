use std::fs::read_to_string;



fn get_first_digit(line: &str) -> char {
    for c in line.chars() {
        if c.is_ascii_digit() {
            return c;
        }
    }
    'a'
}

fn main() {
    let filename = "data.txt";

    let mut sum = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let first = get_first_digit(line);
        let last = get_first_digit(line.chars().rev().collect::<String>().as_str());

        println!("Line: {}", line);
        println!("Found value: {}{}", first, last);

        // Add to the sum the number gained from putting the first and last digits together
        sum += format!("{}{}", first, last).parse::<i32>().unwrap();
    }
    println!("Sum: {}", sum);
    
}
