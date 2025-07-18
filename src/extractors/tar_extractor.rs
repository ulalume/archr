use anyhow::Result;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use tar::Archive;
use xz2::read::XzDecoder;
use bzip2::read::BzDecoder;

// Import the i18n macro
use rust_i18n::t;

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
    pb.set_message(format!("{}", t!("progress.extracting_tar")));
    
    archive.unpack(extract_dir)?;
    
    pb.finish_with_message(format!("{}", t!("progress.extracting_tar")));
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
    pb.set_message(format!("{}", t!("progress.extracting_tar_gz")));
    
    archive.unpack(extract_dir)?;
    
    pb.finish_with_message(format!("{}", t!("progress.extracting_tar_gz")));
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
    pb.set_message(format!("{}", t!("progress.extracting_tar_xz")));
    
    archive.unpack(extract_dir)?;
    
    pb.finish_with_message(format!("{}", t!("progress.extracting_tar_xz")));
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
    pb.set_message(format!("{}", t!("progress.extracting_tar_bz2")));
    
    archive.unpack(extract_dir)?;
    Ok(())
}
