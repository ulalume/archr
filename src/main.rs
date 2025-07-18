use anyhow::{anyhow, Result};
use clap::Parser;
use log::{error, info};
use rfd::MessageDialog;
use std::path::{Path, PathBuf};

mod extractors;
use extractors::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 解凍する圧縮ファイルのパス
    files: Vec<PathBuf>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let args = Args::parse();
    
    // 引数が空の場合、ファイルダイアログを表示
    let files_to_extract = if args.files.is_empty() {
        match select_files().await {
            Some(files) => files,
            None => {
                info!("ファイルが選択されませんでした");
                return;
            }
        }
    } else {
        args.files
    };

    // 複数ファイルの処理
    for file_path in files_to_extract {
        if let Err(e) = extract_archive(&file_path).await {
            error!("解凍に失敗しました: {} - {}", file_path.display(), e);
            show_error_dialog(&format!("解凍に失敗しました: {}\n\nエラー: {}", file_path.display(), e));
        } else {
            info!("✅ 解凍完了: {}", file_path.display());
        }
    }
}

async fn select_files() -> Option<Vec<PathBuf>> {
    let files = rfd::FileDialog::new()
        .add_filter("圧縮ファイル", &["zip", "7z", "rar", "tar", "gz", "xz", "bz2", "tgz", "tar.gz", "tar.xz", "tar.bz2"])
        .set_title("解凍するファイルを選択")
        .pick_files()?;
    
    Some(files)
}

async fn extract_archive(file_path: &Path) -> Result<()> {
    if !file_path.exists() {
        return Err(anyhow!("ファイルが存在しません: {}", file_path.display()));
    }

    // 解凍先ディレクトリを決定（ファイルと同じディレクトリ）
    let parent_dir = file_path.parent()
        .ok_or_else(|| anyhow!("親ディレクトリを取得できません"))?;
    
    let file_stem = file_path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("ファイル名を取得できません"))?;
    
    // .tar.gz のような複合拡張子も考慮
    let extract_dir_name = if file_stem.ends_with(".tar") {
        &file_stem[..file_stem.len() - 4]
    } else {
        file_stem
    };
    
    let mut extract_dir = parent_dir.join(extract_dir_name);
    
    // 同名ディレクトリが存在する場合、連番をつける
    extract_dir = get_unique_path(extract_dir);
    
    info!("📁 解凍開始: {} → {}", file_path.display(), extract_dir.display());
    
    // ファイル拡張子に基づいて適切な解凍関数を呼び出し
    let extension = get_full_extension(file_path);
    match extension.as_str() {
        "zip" => extract_zip(file_path, &extract_dir)?,
        "7z" => extract_7z(file_path, &extract_dir)?,
        "rar" => extract_rar(file_path, &extract_dir)?,
        "tar" => extract_tar(file_path, &extract_dir)?,
        "tar.gz" | "tgz" => extract_tar_gz(file_path, &extract_dir)?,
        "tar.xz" => extract_tar_xz(file_path, &extract_dir)?,
        "tar.bz2" => extract_tar_bz2(file_path, &extract_dir)?,
        "gz" => extract_gz(file_path, &extract_dir)?,
        "xz" => extract_xz(file_path, &extract_dir)?,
        "bz2" => extract_bz2(file_path, &extract_dir)?,
        _ => return Err(anyhow!("サポートされていない形式です: {}", extension)),
    }
    
    Ok(())
}

fn get_full_extension(path: &Path) -> String {
    let file_name = path.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    
    // .tar.gz, .tar.xz, .tar.bz2 のような複合拡張子をチェック
    if file_name.ends_with(".tar.gz") || file_name.ends_with(".tgz") {
        "tar.gz".to_string()
    } else if file_name.ends_with(".tar.xz") {
        "tar.xz".to_string()
    } else if file_name.ends_with(".tar.bz2") {
        "tar.bz2".to_string()
    } else {
        path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase()
    }
}

fn get_unique_path(mut path: PathBuf) -> PathBuf {
    let original_path = path.clone();
    let mut counter = 1;
    
    while path.exists() {
        let file_name = original_path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("extracted");
        
        let new_name = format!("{} ({})", file_name, counter);
        path = original_path.with_file_name(new_name);
        counter += 1;
    }
    
    path
}

fn show_error_dialog(message: &str) {
    MessageDialog::new()
        .set_title("解凍エラー")
        .set_description(message)
        .set_level(rfd::MessageLevel::Error)
        .show();
}
