use std::borrow::Cow;
use std::fs::File;
use std::time::Instant;
use std::io::{prelude::*, self};

pub fn run(from: &String, to: &String, file_path: &String, new_path: &String) -> io::Result<()> {
    let start = Instant::now();
   
    let mut file = File::open(&file_path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);

    let result = find_matches_and_replace(contents, from, to);

    create_file_and_put_contents(result, new_path)?;

    let duration = start.elapsed();
    println!("Done in: {:#?}", duration);

    Ok(())
}

fn find_matches_and_replace(contents: Cow<str>, from: &String, to: &String) -> String {
    let string_matches = contents.matches(&*from).into_iter().count();
    println!("Found {:#?} Matches", string_matches);

    contents.replace(&*from, &to)
}

fn create_file_and_put_contents(content_to_write: String, new_path: &String) -> io::Result<()> {
    println!("Creating new file");
    let mut new_file = File::create(new_path)?;
    println!("Saving new file");
    new_file.write_all(content_to_write.as_bytes())?;

    Ok(())
}