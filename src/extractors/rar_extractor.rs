use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use unrar::Archive;

pub fn extract_rar(file_path: &Path, extract_dir: &Path) -> Result<()> {
    // unrar クレートを使用した実装
    std::fs::create_dir_all(extract_dir)?;
    
    let mut archive = Archive::new(file_path.to_str().unwrap()).open_for_processing()?;
    
    // プログレスバーの設定
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {elapsed_precise} {msg}")
        .unwrap());
    pb.set_message("RARファイルを解凍中...");
    
    loop {
        match archive.read_header() {
            Ok(Some(header)) => {
                let entry = header.entry();
                let output_path = extract_dir.join(entry.filename.as_path());
                
                // プログレスバーのメッセージを更新
                if let Some(file_name) = entry.filename.as_path().file_name().and_then(|s| s.to_str()) {
                    pb.set_message(format!("解凍中: {}", file_name));
                }
                
                if entry.is_directory() {
                    std::fs::create_dir_all(&output_path)?;
                    archive = header.skip()?;
                } else {
                    if let Some(parent) = output_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    
                    let (data, next_archive) = header.read()?;
                    std::fs::write(&output_path, data)?;
                    archive = next_archive;
                }
            }
            Ok(None) => break,
            Err(e) => return Err(e.into()),
        }
    }
    
    pb.finish_with_message("RAR解凍完了!");
    Ok(())
}
