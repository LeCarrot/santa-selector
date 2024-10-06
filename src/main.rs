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

    let mut indexes = (0..gifters.len()).collect();

    index

    // For each Gifter, assign a reference to another gifter
    for i in 0..gifters.len() {
        let rand = rand::thread_rng().gen_range(0..gifters.len());

        gifters[i].giftee = Some(rand);
    }
    

    info!("Gifters {:?}", gifters);
}
