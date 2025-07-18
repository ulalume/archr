use anyhow::Result;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use tar::Archive;
use xz2::read::XzDecoder;
use bzip2::read::BzDecoder;

pub fn extract_tar(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut archive = Archive::new(reader);
    
    fs::create_dir_all(extract_dir)?;
    
    // プログレスバーの設定（TARは事前にエントリ数が分からないのでスピナー形式）
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {elapsed_precise} {msg}")
        .unwrap());
    pb.set_message("TARファイルを解凍中...");
    
    archive.unpack(extract_dir)?;
    
    pb.finish_with_message("TAR解凍完了!");
    Ok(())
}

pub fn extract_tar_gz(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let gz_decoder = GzDecoder::new(reader);
    let mut archive = Archive::new(gz_decoder);
    
    fs::create_dir_all(extract_dir)?;
    
    // プログレスバーの設定
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {elapsed_precise} {msg}")
        .unwrap());
    pb.set_message("TAR.GZファイルを解凍中...");
    
    archive.unpack(extract_dir)?;
    
    pb.finish_with_message("TAR.GZ解凍完了!");
    Ok(())
}

pub fn extract_tar_xz(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let xz_decoder = XzDecoder::new(reader);
    let mut archive = Archive::new(xz_decoder);
    
    fs::create_dir_all(extract_dir)?;
    
    // プログレスバーの設定
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {elapsed_precise} {msg}")
        .unwrap());
    pb.set_message("TAR.XZファイルを解凍中...");
    
    archive.unpack(extract_dir)?;
    
    pb.finish_with_message("TAR.XZ解凍完了!");
    Ok(())
}

pub fn extract_tar_bz2(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let bz_decoder = BzDecoder::new(reader);
    let mut archive = Archive::new(bz_decoder);
    
    fs::create_dir_all(extract_dir)?;
    
    // プログレスバーの設定
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {elapsed_precise} {msg}")
        .unwrap());
    pb.set_message("TAR.BZ2ファイルを解凍中...");
    
    archive.unpack(extract_dir)?;
    
    pb.finish_with_message("TAR.BZ2解凍完了!");
    Ok(())
}
