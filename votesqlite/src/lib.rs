use reqwest::blocking::get;
use std::fs;
use std::io::Write;
use std::path::Path;
use zip::read::ZipArchive;
use std::fs::File;


fn extract_zip(url: &str, directory: &str) -> std::io::Result<String> {
    // Creating directory if not present
    let path = Path::new(directory);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    // Creating filepath for zipped file
    let zip_filepath = path.join("downloaded_file.zip");

    // Downloading zipped file
    let response = get(url).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let mut file = File::create(&zip_filepath)?;
    file.write_all(&response.bytes().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?)?;

    // Extracting zip file
    let file = File::open(&zip_filepath)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(path)?;

    // Removing zipped file after extraction
    fs::remove_file(zip_filepath)?;

    Ok(directory.to_string())
}

