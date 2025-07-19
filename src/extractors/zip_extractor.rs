use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

// Import the i18n macro
use rust_i18n::t;

// Import common decode function
use super::common::decode_filename;

pub fn extract_zip(file_path: &Path, extract_dir: &Path) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    fs::create_dir_all(extract_dir)?;

    // プログレスバーの設定
    let pb = ProgressBar::new(archive.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .unwrap()
        .progress_chars("#>-"));
    
    pb.set_message(format!("{}", t!("progress.extracting_zip")));

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        
        // ファイル名の文字エンコーディングを処理
        let file_name = {
            // 常に生のバイト列からファイル名を取得し、適切にデコード
            let raw_name = file.name_raw();
            let decoded_name = decode_filename(raw_name);
            PathBuf::from(decoded_name)
        };
        
        let outpath = extract_dir.join(&file_name);
        
        // プログレスバーのメッセージを更新
        if let Some(file_name_str) = file_name.file_name().and_then(|s| s.to_str()) {
            pb.set_message(format!("{}", t!("progress.extracting_file", file = file_name_str)));
        }

        if file.name().ends_with('/') {
            // ディレクトリ
            fs::create_dir_all(&outpath)?;
        } else {
            // ファイル
            if let Some(p) = outpath.parent() {
                fs::create_dir_all(p)?;
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // ファイル権限を設定 (Unix系のみ)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
        
        pb.inc(1);
    }
    Ok(())
}
