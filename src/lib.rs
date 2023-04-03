pub mod config;
mod backup;
mod worker;

use std::fs::File;
use std::{process, thread};
use std::time::Instant;
use std::io::{prelude::*, self};

use config::Config;

pub fn run(config: Config) -> io::Result<()> {
    let start = Instant::now();
    let config_for_new_thread = config.clone();
    let mut file = File::open(&config.file_path)?;
    
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);

    let copied_contents = buf.to_owned();

    if worker::find_matches(&contents, &config.from) == 0 {
        println!("Exiting...");
        process::exit(0)
    };

    let handle = thread::spawn(move || {
        let contents_for_new_thred = String::from_utf8_lossy(&copied_contents);

        if !config_for_new_thread.replace {
            println!("Saving backup file..");
        
            if config_for_new_thread.compress {
                backup::create_compressed(&config_for_new_thread.file_path, &contents_for_new_thred).unwrap()
            } else {
                backup::create(&config_for_new_thread.file_path).unwrap()
            }
        }
    });


    let result = worker::replace(contents, &config.from, &config.to);

    worker::create_file_and_put_contents(result, &config.file_path)?;

    handle.join().unwrap();

    let duration = start.elapsed();
    println!("Done in: {:#?}", duration);

    Ok(())
}
