#![feature(hash_extract_if)]

use std::time::Instant;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    start_column: i32,
    end_column: i32,
    row: i32,
    number: i32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Symbol {
    row: i32,
    column: i32,
    symbol: char,  // make it easier to debug
}

fn main() {
    let input = include_str!("../input.txt").to_owned();
    let now = Instant::now();

    let mut sum = 0;
    let mut symbols: HashSet<Symbol> = HashSet::new();
    let mut numbers: HashSet<Number> = HashSet::new();
    let mut extracted_numbers: HashSet<Number> = HashSet::new();

    let mut i = 0;
    for line in input.lines() {
        let mut j = 0;
        let mut num_str = String::from("");
        for c in line.chars() {
            if c.is_digit(10) {
                num_str += &c.to_string();
            } else {
                // Clear the num_str if it exists
                if num_str != "" {
                    // Add the num_str as a number
                    numbers.insert(Number { start_column: j - num_str.len() as i32, end_column: j - 1, row: i, number: num_str.parse::<i32>().unwrap() });
                    num_str = String::from("");
                }

                if c == '*' {
                    // Store the location of the symbol
                    symbols.insert(Symbol { row: i, column: j, symbol: c });
                }
            }

            j += 1;
        }

        // At this point, we've reached the end of the line
        // If there's anything within num_str, we can parse it
        if num_str != "" {
            // Add the num_str as a number
            numbers.insert(Number { start_column: j - num_str.len() as i32, end_column: j - 1, row: i, number: num_str.parse::<i32>().unwrap() });
        }

        i += 1;
    }

    for symbol in &symbols {
        // Check if there's any numbers around the symbol
        // If there is, add it to the sum

        let extracted: Vec<Number> = numbers.extract_if(|x| ((x.start_column - symbol.column).abs() <= 1 || (x.end_column - symbol.column).abs() <= 1) && (x.row - symbol.row).abs() <= 1).collect();

        // If there are exactly 2 numbers adjacent, add the multiple
        if extracted.len() == 2 {
            sum += extracted[0].number * extracted[1].number;
        }
    }

    for num in extracted_numbers {
        print!("{:?} ", num);
    }
    println!();

    for num in numbers {
        print!("{:?} ", num);
    }
    println!();
    println!("{:?}", sum);

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
