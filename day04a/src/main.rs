use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt").to_owned();
    let now = Instant::now();
    let mut sum = 0;

    let mut winning_nums: HashSet<i32> = HashSet::new();
    for line in input.lines() {
        let mut multiple = 0;
        // Throw away everything after the first 8 characters
        let mut card_iter = line.split(": ").skip(1).next().unwrap().split(" | ");
        for num in card_iter.next().unwrap().split(' ') {
            if num != "" {
                winning_nums.insert(num.parse::<i32>().unwrap());
            }
        }
        for num in card_iter.next().unwrap().split(' ') {
            if num != "" && winning_nums.contains(&num.parse::<i32>().unwrap()) {
                if multiple == 0 {
                    multiple = 1;
                } else {
                    multiple *= 2;
                }
            }
        }

        sum += multiple;
        winning_nums.clear();
    }

    println!("{:?}", sum);

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
