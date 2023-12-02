use priority_queue::DoublePriorityQueue;
use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt").to_owned() + "\n";
    let now = Instant::now();

    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut sum = 0;
    for line in input.lines() {
        // Priority queue where the value represents the digit to be added,
        // and the priority is the position in the given string
        let mut pq: DoublePriorityQueue<usize, usize> = DoublePriorityQueue::new();

        for digit_index in 1..10 {
            let index = line.find(digits[digit_index]);
            if index.is_some() {
                pq.push(digit_index, index.unwrap());
            }
        }

        for word_index in 1..10 {
            let index = line.find(words[word_index]);
            if index.is_some() {
                pq.push(word_index, index.unwrap());
            }
        }

        // Get the elements in the priority queue
        // The min element is the first digit, the max element
        // is the last digit
        if pq.len() > 0 {
            sum += pq.peek_min().unwrap().0 * 10;
            sum += pq.peek_max().unwrap().0;
        }
    }

    println!("{sum}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
