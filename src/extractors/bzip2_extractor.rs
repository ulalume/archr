use anyhow::Result;
use bzip2::read::BzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

pub fn extract_bz2(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut decoder = BzDecoder::new(reader);
    
    fs::create_dir_all(extract_dir)?;
    
    // プログレスバーの設定
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {elapsed_precise} {msg}")
        .unwrap());
    pb.set_message("BZ2ファイルを解凍中...");
    
    // .bz2 ファイルの元のファイル名を取得
    let output_name = file_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("extracted");
    
    let output_path = extract_dir.join(output_name);
    let mut output_file = File::create(output_path)?;
    
    std::io::copy(&mut decoder, &mut output_file)?;
    
    pb.finish_with_message("BZ2解凍完了!");
    Ok(())
}
