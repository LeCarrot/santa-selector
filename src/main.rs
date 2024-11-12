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

    info!("Gifters {:?}", gifters);
    info!("Giftees {:?}", giftees);
}
