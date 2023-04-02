use std::{process, io::ErrorKind};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    from: String,

    #[arg(index = 2)]
    to: String,

    #[arg(index = 3)]
    path: String,

    #[arg(index = 4)]
    new_path: String,

    #[arg(short, long, default_value_t = 1)]
    speed: i32,
}
fn main() {
    let args = Args::parse();

    if args.speed > 3 {
        eprintln!("Please set -s (-speed) to either 1, 2 or 3.");
        process::exit(0);
    }

    if let Err(error) = qrep::run(&args.from, &args.to, &args.path, &args.new_path, &args.speed) {

        match error.kind() {
            ErrorKind::NotFound => eprintln!("{:?} does not exists.", &args.path),
            _ =>  eprintln!("An error has occured: {:#?}", error),
        }
       
        process::exit(0);
    }
}