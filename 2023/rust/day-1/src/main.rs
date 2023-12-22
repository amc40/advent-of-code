use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn find_first_digit(line: &str) -> char {
    line.chars()
        .find(|c| c.is_digit(10))
        .expect("should have at least one digit")
}

fn find_last_digit(line: &str) -> char {
    line.chars().rev()
        .find(|c| c.is_digit(10))
        .expect("should have at least one digit")
}

fn main() {
    let lines = lines_from_file("./day-1-input.txt");
    let mut total = 0;
    for line in lines { 
        total += vec![find_first_digit(&line), find_last_digit(&line)]
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
    }
    println!("total: {}", total);
}