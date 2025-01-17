use std::fs::File;
use std::io;
use tar::Archive;
use flate2::read::GzDecoder;

pub fn decompress(input: &str, output: &str) -> io::Result<()> {
    let gzip_file = File::open(input)?;
    let mut decoder = GzDecoder::new(gzip_file);
    let mut tar_file = File::create(output)?;

    io::copy(&mut decoder, &mut tar_file)?;
    Ok(())
}

pub fn extract(tar_path: &str, dest: &str) -> io::Result<()> {
    let file = File::open(tar_path)?;
    let mut archive = Archive::new(file);
    archive.unpack(dest)?;
    Ok(())
}
