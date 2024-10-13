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


#[derive(Debug)]
struct Gifter {
    name: String,
    giftee: Option<usize>,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let gifters = csv_to_gifters(args.gifters);

    if gifters.len() == 1 {
        panic!("Must have more than 1 gifter");
    }

    let gifters = assign_gifters(gifters);

    info!("Gifters {:?}", gifters);
}

fn csv_to_gifters(csv: String) -> Vec<Gifter> {
    let mut gifters: Vec<Gifter> = vec![];

    let args = csv.split(",");
    
    // For each name given, make it into a Gifter
    for g in args {
        gifters.push(Gifter {
            name: g.trim().to_string(),
            giftee: None,
        });
    }

    gifters
}

fn assign_gifters(mut gifters: Vec<Gifter>) -> Vec<Gifter> {
    let mut indexes: Vec<usize> = ( 0..gifters.len() ).collect();

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

    gifters
}
