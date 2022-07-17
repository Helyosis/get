use exitfailure::ExitFailure;
use zip;
use std::path::PathBuf;
use std::fs::File;

pub fn extract(filename: &PathBuf) -> Result<(), ExitFailure> {
    // TODO: Find another way to display the filename
    println!("Extracting {}", filename.as_path().as_os_str().to_str().unwrap());

    let file = File::open(&filename)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut dir_name = filename.clone();
    dir_name.pop();
    
    archive.extract(dir_name)?;

    Ok(())
}