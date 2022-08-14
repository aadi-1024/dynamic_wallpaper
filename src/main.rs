use dynamic_wallpaper::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Please provide all arguments");
        std::process::exit(1);
    }
    run(args);
}

// all logic is in lib.rs
