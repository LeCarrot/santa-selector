use clap::Parser;
use log::debug;
use secret_santa::*;
use std::fs;
use std::io::ErrorKind;

#[derive(Parser)]
struct Args {
    gifters: String, // one string with comma-seperated names
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let gifters: Vec<&str> = args.gifters.split(",").map(|x| x.trim()).collect();

    let matching = |x: &Vec<&str>| -> bool {
        gifters.iter()
            .zip(x.iter())
            .filter(|&(a, b)| a == b)
            .count() > 0
    };

    let giftees = santa_shuffle(&gifters, matching);
    let pairs: Vec<_> = gifters.iter().zip(giftees.iter()).collect();

    debug!("Pairs {:?}", pairs);

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
