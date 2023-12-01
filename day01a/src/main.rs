use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt").to_owned() + "\n";
    let now = Instant::now();

    let mut sum = 0;
    for line in input.split('\n') {
        let mut num_str = String::from("");
        for c in line.split("") {
            let n = c.parse::<i32>();
            if n.is_ok() {
                num_str += c;
            }
        }

        if !num_str.is_empty() {
            let num_str_count = num_str.chars().count();
            sum += num_str.get(0..1).unwrap().parse::<i32>().unwrap() * 10;
            sum += num_str.get((num_str_count - 1)..num_str_count).unwrap().parse::<i32>().unwrap();
        }
    }

    println!("{sum}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
