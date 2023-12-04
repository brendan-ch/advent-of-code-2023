use priority_queue::DoublePriorityQueue;
use std::time::Instant;

// Work around Rust priority queue shenanigans
#[derive(PartialEq, Eq, Hash, Debug)]
struct Element {
    digit: usize,
    priority: usize,
}

fn main() {
    let input = include_str!("../input.txt").to_owned() + "\n";
    let now = Instant::now();

    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut sum = 0;
    for line in input.lines() {
        // Priority queue where the value represents the digit to be added,
        // and the priority is the position in the given string
        let mut pq: DoublePriorityQueue<Element, usize> = DoublePriorityQueue::new();

        for digit_index in 1..10 {
            let indices = line.match_indices(digits[digit_index]);
            // We only care about the first and last indices
            let mut iter = indices.into_iter();
            let next = iter.next();

            if next.is_some() {
                pq.push(Element { digit: digit_index, priority: next.unwrap().0 }, next.unwrap().0);
            }

            let last = iter.last();

            if last.is_some() {
                pq.push(Element { digit: digit_index, priority: last.unwrap().0 }, last.unwrap().0);
            }
        }

        for word_index in 1..10 {
            let indices = line.match_indices(words[word_index]);
            
            // We only care about the first and last indices
            let mut iter = indices.into_iter();
            let next = iter.next();

            if next.is_some() {
                pq.push(Element { digit: word_index, priority: next.unwrap().0 }, next.unwrap().0);
            }

            let last = iter.last();

            if last.is_some() {
                pq.push(Element { digit: word_index, priority: last.unwrap().0 }, last.unwrap().0);
            }
        }

        // Get the elements in the priority queue
        // The min element is the first digit, the max element
        // is the last digit
        if pq.len() > 0 {
            sum += pq.peek_min().unwrap().0.digit * 10;
            sum += pq.peek_max().unwrap().0.digit;
        }
    }

    println!("{sum}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
