use std::fs::{self, File};

use std::process;
use std::time::Instant;
use std::io::{prelude::*, self};

use indicatif::ProgressBar;

pub fn run(from: &String, to: &String, file_path: &String, speed: &i32) -> io::Result<()> {
    let contents = fs::read_to_string(&file_path)?;

    let start = Instant::now();

    let result = {
        match speed {
            1 => { contents.replace(&*from, &to ) },
            2 => { 
                let string_matches = contents.matches(&*from).into_iter().count();
                println!("Found {:#?} Matches", string_matches);
                contents.replace(&*from, &to )
            },
            3 => {
                let mut contents = contents;
                let string_matches = contents.matches(&*from).into_iter().count();
                
                println!("Found {:#?} Matches", string_matches);
                let pb = ProgressBar::new(string_matches.try_into().unwrap());

                for _ in 0..string_matches {
                    contents = contents.replacen(&*from, &to, 1); 

                    pb.inc(1);
                }

                pb.finish_with_message("done");
                contents
            }
            _ => {
                println!("Speed cannot be more than 3");
                process::exit(1);
            }
        }
    };

    println!("Creating new file");
    let mut new_file = File::create("replaced.txt")?;
    println!("Saving new file");
    new_file.write_all(result.as_bytes())?;

    let duration = start.elapsed();
    println!("Done in: {:#?}", duration);

    Ok(())
}