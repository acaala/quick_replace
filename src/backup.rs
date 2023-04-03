use std::borrow::Cow;
use std::fs::{File, rename};
use std::io::{Write, Result};
use std::path::Path;

use zip::result::ZipResult;
use zip::{ZipWriter, CompressionMethod};
use zip::write::FileOptions;

pub fn create(file_path: &String) -> Result<()> {
let original_file_path = Path::new(&file_path);

    let backup_file_name = file_path.to_owned() + ".bak";
    let backup_path = Path::new(&backup_file_name);
    
    rename(&original_file_path, &backup_path)?;

    Ok(())
}

pub fn create_compressed(file_path: &String, contents: &Cow<str>) -> ZipResult<()> {
    println!("Compressing...");
    let file_path = Path::new(file_path);
    let zipped_file_name = format!("{}.bak.zip", &file_path.with_extension("").to_str().unwrap()).replace("\"", "");
    
    let file = File::create(zipped_file_name).unwrap();

    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    zip.start_file(file_path.file_name().unwrap().to_str().unwrap(), options)?;
    zip.write_all(&contents.as_bytes())?;

    zip.finish()?;

    Ok(())
}
