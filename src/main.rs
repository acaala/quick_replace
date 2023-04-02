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
}
fn main() {
    let args = Args::parse();

    if let Err(error) = qrep::run(&args.from, &args.to, &args.path, &args.new_path) {

        match error.kind() {
            ErrorKind::NotFound => eprintln!("{:?} does not exists.", &args.path),
            _ =>  eprintln!("An error has occured: {:#?}", error),
        }
       
        process::exit(0);
    }
}