use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt").to_owned() + "\n";
    let now = Instant::now();

    let mut sum = 0;

    for line in input.lines() {
        let game_split = line.split(": ");
        let mut game_split_iter = game_split.into_iter();
        game_split_iter.next();  // no longer need the game id

        let mut r = 0;
        let mut b = 0;
        let mut g = 0;
        
        let set_split_iter = game_split_iter.next().unwrap().split("; ").into_iter();
        for set in set_split_iter {
            let info_split_iter = set.split(", ").into_iter();
            for info in info_split_iter {
                let collected: Vec<&str> = info.split(" ").collect();
                match collected[1] {
                    "blue" => {
                        let b_c = collected[0].parse::<i32>().unwrap();
                        if b_c > b {
                            b = b_c;
                        }
                    }
                    "red" => {
                        let r_c = collected[0].parse::<i32>().unwrap();
                        if r_c > r {
                            r = r_c;
                        }
                    }
                    "green" => {
                        let g_c = collected[0].parse::<i32>().unwrap();
                        if g_c > g {
                            g = g_c;
                        }
                    }
                    _ => {}
                }
            }
        }

        sum += r * g * b;
    }
    println!("{sum}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
