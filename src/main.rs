use std::{process, io::ErrorKind};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Text to be replaced.
    #[arg(index = 1)]
    from: String,

    /// Text to replace with.
    #[arg(index = 2)]
    to: String,

    /// Path to file.
    #[arg(index = 3)]
    file_path: String,

    /// If you dont want to create a backup file.
    #[arg(short, long, default_value_t = false)]
    replace: bool,
}
fn main() {
    let args = Args::parse();

    // TODO: Not this..
    let file_path_clone = args.file_path.clone();

    let config = qrep::Config::build(args.from, args.to, args.file_path, args.replace);

    if let Err(error) = qrep::run(config) {
        match error.kind() {
            ErrorKind::NotFound => eprintln!("{:?} does not exist.", file_path_clone),
            _ =>  eprintln!("An error has occured: {:#?}", error),
        }
       
        process::exit(0);
    }
}