pub mod config;
mod backup;
mod worker;

use std::fs::File;
use std::{process};
use std::time::Instant;
use std::io::{prelude::*, self};

use config::Config;

pub fn run(config: Config) -> io::Result<()> {
    let start = Instant::now();
   
    let mut file = File::open(&config.file_path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);


    if worker::find_matches(&contents, &config.from) == 0 {
        println!("Exiting...");
        process::exit(0)
    };

    if !config.replace {
        println!("Saving backup file..");
       
        if config.compress {
            backup::create_compressed(&config.file_path, &contents)?
        } else {
            backup::create(&config.file_path)?;
        }
    }

    let result = worker::replace(contents, &config.from, &config.to);

    worker::create_file_and_put_contents(result, &config.file_path)?;

    let duration = start.elapsed();
    println!("Done in: {:#?}", duration);

    Ok(())
}
