use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use sevenz_rust::{Password, SevenZReader};
use std::fs::{self, File};
use std::path::Path;

// Import the i18n macro
use rust_i18n::t;

// Import common decode function
use super::common::decode_filename_as_pathbuf;

pub fn extract_7z(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let mut file = File::open(file_path)?;
    let file_size = file.metadata()?.len();
    let mut sz = SevenZReader::new(&mut file, file_size, Password::empty())?;

    fs::create_dir_all(extract_dir)?;

    // プログレスバーの設定（7zも事前にエントリ数が分からないのでスピナー形式）
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {elapsed_precise} {msg}")
        .unwrap());
    pb.set_message(format!("{}", t!("progress.extracting_7z")));

    sz.for_each_entries(|entry, reader| {
        // ファイル名を適切にデコード
        let decoded_name = decode_filename_as_pathbuf(entry.name.as_bytes());
        let entry_path = extract_dir.join(&decoded_name);
        
        // プログレスバーのメッセージを更新
        if let Some(file_name) = decoded_name.file_name().and_then(|s| s.to_str()) {
            pb.set_message(format!("{}", t!("progress.extracting_file", file = file_name)));
        }
        
        if entry.is_directory() {
            fs::create_dir_all(&entry_path)?;
        } else {
            if let Some(parent) = entry_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            let mut output_file = File::create(&entry_path)?;
            std::io::copy(reader, &mut output_file)?;
        }
        
        pb.inc(1);
        Ok(true)
    })?;
    
    Ok(())
}
