use std::fs::{self, File};

use std::time::Instant;
use std::io::{prelude::*, self};

pub fn run(from: &String, to: &String, file_path: &String, speed: &i32) -> io::Result<()> {
    let contents = fs::read_to_string(&file_path)?;

    let start = Instant::now();

    if speed > &1 {
        let matches = contents.matches("test").into_iter().count();
        println!("Found {:#?} Matches", matches);
    }

    let result = contents.replace(&*from, &to);

    let mut new_file = File::create("replaced.txt")?;
    new_file.write_all(result.as_bytes())?;

    let duration = start.elapsed();
    println!("Done in: {:#?}", duration);

    Ok(())
}