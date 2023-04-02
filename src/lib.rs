use std::borrow::Cow;
use std::fs::{File, self};
use std::{process, path};
use std::time::Instant;
use std::path::Path;

use std::io::{prelude::*, self};

use clap::{Arg, Args};

pub fn run(config: Config) -> io::Result<()> {
    let start = Instant::now();
   
    let mut file = File::open(&config.file_path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);


    if find_matches(&contents, &config.from) == 0 {
        println!("Exiting...");
        process::exit(0)
    };

    if !config.replace {
        println!("Saving backup file..");
        create_backup(&config.file_path)?;
    }


    let result = replace(contents, &config.from, &config.to);

    create_file_and_put_contents(result, &config.file_path)?;

    let duration = start.elapsed();
    println!("Done in: {:#?}", duration);

    Ok(())
}


fn find_matches(contents: &Cow<str>, from: &String) -> usize {
    let string_matches = contents.matches(&*from).into_iter().count();
    println!("Found {:#?} instances of {:#?}", string_matches, from);

    string_matches
}

fn replace(contents: Cow<str>, from: &String, to: &String) -> String {

    println!("Replacing {:?} to {:?}..", from, to);
    contents.replace(&*from, &to)
}

fn create_backup(file_path: &String) -> io::Result<()> {
   let original_file_path = Path::new(&file_path);

    let backup_file_name = file_path.to_owned() + ".bak";
    let backup_path = Path::new(&backup_file_name);
    
    fs::rename(&original_file_path, &backup_path)?;

    Ok(())
}

fn create_file_and_put_contents(content_to_write: String, new_path: &String) -> io::Result<()> {
    let mut new_file = File::create(new_path)?;
    println!("Saving to {:?}", new_path);
    new_file.write_all(content_to_write.as_bytes())?;

    Ok(())
}

pub struct Config {
    from: String,
    to: String,
    file_path: String,
    replace: bool,
}

impl Config {
    pub fn build(from: String, to: String, file_path: String, replace: bool) -> Config {
        Config { from, to, file_path, replace }
    }
}