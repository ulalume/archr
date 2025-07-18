use anyhow::Result;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

// Import the i18n macro
use rust_i18n::t;

pub fn extract_gz(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut decoder = GzDecoder::new(reader);
    
    fs::create_dir_all(extract_dir)?;
    
    // プログレスバーの設定
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {elapsed_precise} {msg}")
        .unwrap());
    pb.set_message(format!("{}", t!("progress.extracting_gz")));
    
    // .gz ファイルの元のファイル名を取得
    let output_name = file_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("extracted");
    
    let output_path = extract_dir.join(output_name);
    let mut output_file = File::create(output_path)?;
    
    std::io::copy(&mut decoder, &mut output_file)?;

    Ok(())
}
