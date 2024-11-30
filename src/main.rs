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

    // collect inputs
    let args = Args::parse();
    let gifters: Vec<&str> = args.gifters.split(",").map(|x| x.trim()).collect();

    // "the magic"
    let pairs = gifters.santa_shuffle();
    debug!("Pairs {:?}", pairs);    // (double check "the magic")

    // make results directory
    match fs::create_dir("results") {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => (),
            _ => panic!("Unable to create results directory"),
        }
    };

    // print to files
    for i in 0..pairs.len() {
        let gifter = pairs[i].0;
        let filename = format!("results/{gifter}.txt");
        fs::write(filename, pairs[i].1).expect("Unable to write file");
    }
}
