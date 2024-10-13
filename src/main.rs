/*
* How is this gonna work?
* take in a comma-separated list of names?
*
* We have a vec of strings that is our gifters
*
* The final gifter pool could be a vec of tuples of vec slices?
*
* let gifters = args.gifters.parse_csv; // vector of strings
*
* foreach gifters as gifter
*   let pick = gifters[rand];
*   
*   let giftPair = (gifter, pick)
*
*   foreach behavior
*       execute(giftPair)
*       if flase, regen giftpair
*
* Functions for filter behavior
*   not_itself(giftPair) -> bool
*/

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
