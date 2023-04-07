use std::borrow::Cow;
use std::fs::{File, rename};
use std::io::{Write, Result};
use std::path::Path;
use std::process;

use zip::result::ZipResult;
use zip::{ZipWriter, CompressionMethod};
use zip::write::FileOptions;


/// Creates a backup file from the original
pub fn create(file_path: &String) -> Result<()> {
    let original_file_path = Path::new(&file_path);

    let backup_file_name = file_path.to_owned() + ".bak";
    let backup_path = Path::new(&backup_file_name);
    
    rename(&original_file_path, &backup_path)?;

    Ok(())
}

/// Creates a compressed backup file from the original
pub fn create_compressed(file_path: &String, contents: &Cow<str>) -> ZipResult<()> {
    println!("Compressing...");
    let file_path = Path::new(file_path);
    let zipped_file_name = format!("{}.zip", &file_path.with_extension("").to_str().unwrap()).replace("\"", "");
    
    let file = File::create(zipped_file_name)?;

    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    let compressed_file_name = file_path.file_name().and_then(|f| f.to_str()).unwrap_or_else(|| {
        println!("unexpected error reading filename");
        process::exit(0)
    });

    zip.start_file(compressed_file_name, options)?;
    zip.write_all(&contents.as_bytes())?;

    zip.finish()?;

    Ok(())
}
