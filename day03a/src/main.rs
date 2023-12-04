#![feature(hash_extract_if)]

use std::time::Instant;
use std::collections::HashSet;

// Install nightly Rust using rustup: `rustup default nightly`
// To switch back to stable: `rustup default stable`

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

                if c != '.' {
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

        let extracted = numbers.extract_if(|x| ((x.start_column - symbol.column).abs() <= 1 || (x.end_column - symbol.column).abs() <= 1) && (x.row - symbol.row).abs() <= 1);

        for num in extracted.into_iter() {
            sum += num.number;
        }
    }

    println!("{:?}", sum);

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
