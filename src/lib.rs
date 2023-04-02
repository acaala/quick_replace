use std::fs::File;

use std::process;
use std::time::Instant;
use std::io::{prelude::*, self};

use indicatif::ProgressBar;

pub fn run(from: &String, to: &String, file_path: &String, new_path: &String, speed: &i32) -> io::Result<()> {
    // let contents = fs::read_to_string(&file_path)?;
    let mut file = File::open(&file_path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);

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
                let mut contents = contents.to_string();
                let string_matches = contents.matches(&*from).into_iter().count();
                
                println!("Found {:#?} Matches", string_matches);
                let pb = ProgressBar::new(string_matches.try_into().unwrap());

                for _ in 0..3 {
                    contents = contents.replacen(&*from, &to, string_matches / 3); 

                    pb.inc((string_matches / 3).try_into().unwrap());
                }

                pb.finish_with_message("done");
                contents
            }
            _ => {
                println!("Speed cannot be more than 3");
                process::exit(0);
            }
        }
    };

    println!("Creating new file");
    let mut new_file = File::create(&new_path)?;
    println!("Saving new file");
    new_file.write_all(result.as_bytes())?;

    let duration = start.elapsed();
    println!("Done in: {:#?}", duration);

    Ok(())
}