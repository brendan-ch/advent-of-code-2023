use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt").to_owned();
    let now = Instant::now();
    let mut sum = 0;

    let mut winning_nums: HashSet<i32> = HashSet::new();
    let mut cards: Vec<i32> = Vec::new();
    let mut current = 0;
    
    for line in input.lines() {
        if cards.len() <= current {
            cards.push(1);
        }
        let mut num_winning = 0;
        // Throw away everything after the first 8 characters
        let mut card_iter = line.split(": ").skip(1).next().unwrap().split(" | ");
        for num in card_iter.next().unwrap().split(' ') {
            if num != "" {
                winning_nums.insert(num.parse::<i32>().unwrap());
            }
        }
        // println!("{:?}", winning_num);
        for num in card_iter.next().unwrap().split(' ') {
            if num != "" && winning_nums.contains(&num.parse::<i32>().unwrap()) {
                num_winning += 1;
            }
        }

        for i in (current + 1)..=(current + num_winning) {
            // If the index is in the array
            if cards.len() as i32 <= i as i32 {
                cards.push(1 + cards[current as usize]);
            } else {
                cards[i as usize] += cards[current as usize];
            }
        }

        winning_nums.clear();
        current += 1;
    }
    
    for card in cards {
        sum += card;
    }
    println!("{:?}", sum);

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
