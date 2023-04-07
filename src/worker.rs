use std::{borrow::Cow, fs::File, io::{self, Write}};

/// Finds and returns the number of string matches
pub fn find_matches(contents: &Cow<str>, from: &String) -> usize {
    let string_matches = contents.matches(&*from).into_iter().count();
    println!("Found {:#?} instances of {:#?}", string_matches, from);

    string_matches
}

/// Replaces the from_string to to_string
pub fn replace(contents: Cow<str>, from: &String, to: &String) -> String {
    println!("Replacing {:?} to {:?}..", from, to);
    contents.replace(&*from, &to)
}

/// Creates a new file with the string replaced contents
pub fn create_file_and_put_contents(content_to_write: String, new_path: &String) -> io::Result<()> {
    let mut new_file = File::create(new_path)?;
    println!("Saving to {:?}", new_path);
    new_file.write_all(content_to_write.as_bytes())?;

    Ok(())
}
