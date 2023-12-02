use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt").to_owned() + "\n";
    let now = Instant::now();

    // Constant numbers of cubes possible
    let r = 12;
    let g = 13;
    let b = 14;

    let mut sum = 0;

    for line in input.lines() {
        let game_split = line.split(": ");
        let mut game_split_iter = game_split.into_iter();
        let game_id_str: String = game_split_iter.next().unwrap().chars().skip(5).collect();
        let game_id = game_id_str.parse::<i32>().unwrap();
        
        
        let mut possible = true;

        let set_split_iter = game_split_iter.next().unwrap().split("; ").into_iter();
        for set in set_split_iter {
            let info_split_iter = set.split(", ").into_iter();
            for info in info_split_iter {
                let collected: Vec<&str> = info.split(" ").collect();
                match collected[1] {
                    "blue" => {
                        let b_c = collected[0].parse::<i32>().unwrap();
                        if b_c > b {
                            possible = false;
                        }
                    }
                    "red" => {
                        let r_c = collected[0].parse::<i32>().unwrap();
                        if r_c > r {
                            possible = false;
                        }
                    }
                    "green" => {
                        let g_c = collected[0].parse::<i32>().unwrap();
                        if g_c > g {
                            possible = false;
                        }
                    }
                    _ => {}
                }
            }
        }

        println!("{game_id} {possible}");
        if possible {
            sum += game_id;
        }
    }
    println!("{sum}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
