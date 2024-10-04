/*
* How is this gonna work?
* take in a comma-separated list of names?
*/

use clap::Parser;
use rand::thread_rng;
use rand::seq::SliceRandom;
use log::{info, warn, debug};

#[derive(Parser)]
struct Args {
    gifters: String,
}

struct Gifter {
    name: String,
    giftee: &Gifter,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let args = args.gifters.split(",");

    let mut gifters = vec![];

    for g in args {
        gifters.push(g.trim());
    }

    let mut giftees = gifters.clone();
    let person_count = gifters.len();

    let mut must_loop = true;
    while must_loop {
        giftees.shuffle(&mut thread_rng());
        
        for i in 0..person_count {
            if gifters[i] == giftees[i] {
                debug!("Giftees {:?} Someone got themself. Retrying...", giftees);
                must_loop = true;
            }
            else {
                must_loop = false;
            }
        }
    }

    info!("Gifters {:?}", gifters);
    info!("Giftees {:?}", giftees);
}
