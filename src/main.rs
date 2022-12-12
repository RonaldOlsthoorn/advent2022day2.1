use std::fs::File;
use std::io::{prelude::*, BufReader};


fn calculate_score(own_play: int32, opponent_play: int32)-> int32{

    let mut diff = own_play - opponent_play;  // 1 or -2 for win, -1 or 2 for lose, 0 for draw
    if diff.abs()==2{
        diff *= -1;
        diff /= 2;
    }

    own_play + 3 * (diff + 1)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        
        score += calculate_score(own_play, opponent_play);
    }

    println!("total calories: {}", gold_calories + silver_calories + bronze_calories);
}