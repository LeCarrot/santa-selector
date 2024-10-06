/*
* How is this gonna work?
* take in a comma-separated list of names?
*/

use clap::Parser;
use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;
use log::{info, warn, debug};

#[derive(Parser)]
struct Args {
    gifters: String,
}


#[derive(Debug)]
struct Gifter {
    name: String,
    giftee: Option<usize>,
}



fn main() {
    env_logger::init();

    let args = Args::parse();
    let args = args.gifters.split(",");

    let mut gifters: Vec<Gifter> = vec![];
    
    // For each name given, make it into a Gifter
    for g in args {
        gifters.push(Gifter {
            name: g.trim().to_string(),
            giftee: None,
        });
    }

    if gifters.len() == 1 {
        panic!("Must have more than 1 gifter");
    }

    let mut indexes: Vec<usize> = (0..gifters.len()).collect();

    let mut done = false;
    while !done {
        debug!("Shuffle!");
        indexes.shuffle(&mut thread_rng());
        
        for i in 0..gifters.len() {

            if indexes[i] == i {
                // reshuffle and start over
                done = false;
                break;
            }
            else {
                done = true;
                gifters[i].giftee = Some(indexes[i]);
            }
        
        }

    }
    

    info!("Gifters {:?}", gifters);
}
