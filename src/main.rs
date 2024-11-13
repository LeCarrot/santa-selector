use clap::Parser;
use rand::thread_rng;
use rand::seq::SliceRandom;
use log::{info, debug};
use std::fs;
use std::io::ErrorKind;

#[derive(Parser)]
struct Args {
    gifters: String,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let gifters: Vec<&str> = args.gifters
        .split(",")
        .map(|x| x.trim())
        .collect();

    let mut giftees: Vec<&str> = gifters.clone();

    loop {
        giftees.shuffle(&mut thread_rng());
        let matching = gifters
            .iter()
            .zip(giftees.iter())
            .filter(|&(a, b)| a == b)
            .count();

        if matching == 0 || giftees.len() <= 1 {
            break;
        }
    }

    debug!("Gifters {:?}", gifters);
    debug!("Giftees {:?}", giftees);


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
}
