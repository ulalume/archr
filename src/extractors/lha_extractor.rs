use anyhow::Result;
use encoding_rs::SHIFT_JIS;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

// Import the i18n macro
use rust_i18n::t;

pub fn extract_lha(file_path: &Path, extract_dir: &Path) -> Result<()> {
    fs::create_dir_all(extract_dir)?;
    
    // プログレスバーの設定
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {elapsed_precise} {msg}")
        .unwrap());
    pb.set_message(format!("{}", t!("progress.extracting_lha")));
    
    // アーカイブファイルを読み込み
    let archive_data = std::fs::read(file_path)?;
    let mut cursor = std::io::Cursor::new(archive_data);
    let mut extracted_files = 0;
    
    // delharc 0.6.1の新しいAPIを使用してアーカイブ全体を処理
    loop {
        // 各ファイルエントリを解凍
        match delharc::LhaDecodeReader::new(&mut cursor) {
            Ok(mut decoder) => {
                let header = decoder.header();
                let filename = decode_filename(&header.filename);
                let output_path = extract_dir.join(&filename);
                
                pb.set_message(format!("{}", t!("progress.extracting_file", file = filename.to_string_lossy())));
                
                if header.is_directory() {
                    // ディレクトリの作成
                    fs::create_dir_all(&output_path)?;
                } else {
                    // ファイルの解凍
                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    
                    // ファイル内容を解凍して書き込み
                    let mut content = Vec::new();
                    std::io::copy(&mut decoder, &mut content)?;
                    
                    let mut output_file = File::create(&output_path)?;
                    output_file.write_all(&content)?;
                }
                
                extracted_files += 1;
                pb.inc(1);
                
                // デコーダー処理後の位置を確認
                let current_pos = cursor.position();
                let total_len = cursor.get_ref().len() as u64;
                
                // アーカイブの終端に達している場合は終了
                if current_pos >= total_len {
                    break;
                }
            }
            Err(_) => {
                // エラーまたはアーカイブの終端
                if extracted_files == 0 {
                    pb.finish_with_message("LHA extraction failed");
                    return Err(anyhow::anyhow!("有効なLHAファイルが見つかりませんでした"));
                }
                // すでにファイルを解凍している場合は正常終了
                break;
            }
        }
    }
    
    if extracted_files > 0 {
        pb.finish_with_message(format!("{} - {} files extracted", t!("progress.extracting_lha"), extracted_files));
        Ok(())
    } else {
        pb.finish_with_message("LHA extraction failed");
        Err(anyhow::anyhow!("LHAアーカイブからファイルを解凍できませんでした"))
    }
}

/// ファイル名のデコード（日本語対応）
fn decode_filename(raw_bytes: &[u8]) -> PathBuf {
    // まずUTF-8として試す
    if let Ok(utf8_str) = std::str::from_utf8(raw_bytes) {
        return PathBuf::from(utf8_str);
    }
    
    // UTF-8でない場合、Shift_JISとしてデコード
    let (decoded, _, _) = SHIFT_JIS.decode(raw_bytes);
    PathBuf::from(decoded.into_owned())
}
