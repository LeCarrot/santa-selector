use clap::Parser;
use rand::thread_rng;
use rand::seq::SliceRandom;
use log::{info, debug};

#[derive(Parser)]
struct Args {
    gifters: String,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let mut gifters: Vec<String> = vec![];

    for arg in args.gifters.split(",") {
        gifters.push(arg.trim().to_string());
    }

    if gifters.len() == 1 {
        panic!("Must have more than 1 gifter");
    }

    let mut pair_index: Vec<usize> = ( 0..gifters.len() ).collect();

    let mut gift_pairs: Vec<(String, String)> = vec![];

    let mut done = false;
    while !done {
        debug!("Shuffle!");
        pair_index.shuffle(&mut thread_rng());

        for i in 0..gifters.len() {
            if pair_index[i] == i {
                done = false;
                gift_pairs = vec![];
                break;
            }
            else {
                done = true;
                gift_pairs.push((gifters[i].clone(), gifters[pair_index[i]].clone()));
            }
        }
    }

    info!("Gifters {:?}", gift_pairs);
}
