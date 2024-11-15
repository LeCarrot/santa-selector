use clap::Parser;
use rand::thread_rng;
use rand::seq::SliceRandom;
use log::{info, debug};
use std::fs;
use std::io::ErrorKind;
// use std::Vec;

#[derive(Parser)]
struct Args {
    gifters: String,
}

trait SantaShuffle<T> {
    fn santa_shuffle(&self) -> Vec<T>;
}

impl<T: std::cmp::PartialEq + std::clone::Clone> SantaShuffle<T> for Vec<T> {
    fn santa_shuffle(&self) -> Vec<T> {
        let mut new_vec = self.clone();

        new_vec.shuffle(&mut thread_rng());

        // the rest of this stuff really breaks down if the vector only has 0
        // or 1 elements in it. In that case, let's just make this a regular
        // shuffle
        if new_vec.len() <= 1 {
            return new_vec;
        }

        let matching_elements = self.iter()
            .zip(new_vec.iter())
            .filter(|&(a, b)| a == b)
            .count();

        if matching_elements > 0 {
            new_vec = new_vec.santa_shuffle();
        }

        return new_vec.to_vec();
    }
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let gifters: Vec<&str> = args.gifters.split(",").map(|x| x.trim()).collect();

    let giftees = gifters.santa_shuffle();

    let pairs: Vec<_> = gifters.iter().zip(giftees.iter()).collect();

    debug!("Gifters {:?}", gifters);
    debug!("Giftees {:?}", giftees);
    debug!("Pairs {:?}", pairs);
/*
    // make results directory
    match fs::create_dir("results") {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => (),
            _ => panic!("Unable to create results directory"),
        }
    };

    // print to files
    for i in 0..gifters.len() {
        let gifter = gifters[i];
        let filename = format!("results/{gifter}.txt");
        fs::write(filename, giftees[i]).expect("Unable to write file");
    }
*/
}
