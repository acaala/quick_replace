use std::{process, io::{Read}, time::Instant, fs::File, error::Error};
use clap::Parser;
use qrep::{worker, backup};

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

    /// If you dont want to create a backup file (cannot be used with -c (--compress)).
    #[arg(short, long, default_value_t = false)]
    replace: bool,

    /// Compress the backup file (cannot be used with -r (--replace))
    #[arg(short, long, default_value_t = false)]
    compress: bool,
}

fn main() {
    let args = Args::parse();

    if args.compress && args.replace {
        println!("Cannot use -c (--compress) with -r (--replace)");
        process::exit(0);
    }

    if let Err(error) = run_task(args) {
        println!("{}", error);
        process::exit(0);
    }
}

fn run_task(args: Args) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let mut file = File::open(&args.file_path).unwrap_or_else(|e| {
        println!("Error opening file: {}", e);
        process::exit(0);
    });

    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);

    if worker::find_matches(&contents, &args.from) == 0 {
        println!("Exiting...");
        process::exit(0)
    }

    if !args.replace {
        println!("Saving backup file..");

        if args.compress {
            backup::create_compressed(&args.file_path, &contents)?;
        } else {
            backup::create(&args.file_path)?;
        }
    }

    let result = worker::replace(contents, &args.from, &args.to);

    worker::create_file_and_put_contents(result, &args.file_path)?;

    let duration = start.elapsed();
    println!("Done in: {:#?}", duration);

    Ok(())
}