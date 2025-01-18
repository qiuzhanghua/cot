use std::fs::File;
use std::io;
use std::path::PathBuf;
use zip::ZipArchive;

pub fn unzip(zip_path: &str, dest: &str) -> anyhow::Result<()> {
    let zip_file = File::open(zip_path)?;

    let mut archive = ZipArchive::new(zip_file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;

        let mut extract_path = PathBuf::from(dest);
        extract_path.push(entry.name());

        if let Some(parent) = extract_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if !entry.is_dir() {
            // println!("Extracting: {}", extract_path.display());
            let mut output_file = File::create(extract_path)?;
            std::io::copy(&mut entry, &mut output_file)?;
        } else {
            // println!("Creating directory: {}", extract_path.display());
            std::fs::create_dir_all(extract_path)?;
        }
    }
    Ok(())
}
