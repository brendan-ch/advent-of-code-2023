use fancy_regex::Captures;
use fancy_regex::Match;
use fancy_regex::Regex;
use std::string::String;
use std::time::Instant;
use std::vec::Vec;

fn parse_match(m: &Captures<'_>) -> i32 {
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    println!("parse_match called");

    let capture = m.iter().next().unwrap().unwrap();
    
    let as_str = capture.as_str();
    println!("{as_str}");
    let mut matching: String = as_str.chars().skip(capture.start()).collect();
    println!("{matching} {} {as_str} {:?}", capture.start(), capture);
    matching = matching[0..capture.end()].to_string();

    let position = numbers.iter().position(|&n| n == matching);
    let next = matching.chars().next();

    if position.is_some() {
        return position.unwrap() as i32;
    } else if next.is_some() && next.unwrap().is_digit(10) {
        return next.unwrap().to_digit(10).unwrap() as i32;
    }

    return 0;
}

fn main() {
    let input = include_str!("../input.txt").to_owned() + "\n";
    let now = Instant::now();

    

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
