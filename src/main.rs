use anyhow::{anyhow, Result};
use clap::Parser;
use log::{error, info};
use rfd::MessageDialog;
use std::path::{Path, PathBuf};

// Initialize rust-i18n
rust_i18n::i18n!("locales", fallback = "en");
use rust_i18n::t;

mod extractors;
use extractors::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Archive files to extract / 解凍する圧縮ファイルのパス
    files: Vec<PathBuf>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // Set locale based on system language, default to English
    let locale = if std::env::var("LANG").unwrap_or_default().contains("ja")
        || std::env::var("LC_ALL").unwrap_or_default().contains("ja")
        || std::env::var("LANGUAGE").unwrap_or_default().contains("ja")
    {
        "ja"
    } else {
        "en"
    };
    rust_i18n::set_locale(locale);

    let args = Args::parse();

    // 引数が空の場合、ファイルダイアログを表示
    let files_to_extract = if args.files.is_empty() {
        match select_files().await {
            Some(files) => files,
            None => {
                info!("{}", t!("ui.no_files_selected").to_string());
                return;
            }
        }
    } else {
        args.files
    };

    // 複数ファイルの処理
    for file_path in files_to_extract {
        if let Err(e) = extract_archive(&file_path).await {
            let error_msg = t!(
                "ui.extraction_failed",
                file = file_path.display(),
                error = e
            );
            error!("{}", error_msg);
            show_error_dialog(&error_msg);
        } else {
            let success_msg = t!("ui.extraction_complete", file = file_path.display());
            info!("{}", success_msg);
        }
    }
}

async fn select_files() -> Option<Vec<PathBuf>> {
    let files = rfd::FileDialog::new()
        .add_filter(
            &t!("app.description").to_string(),
            &[
                "zip", "7z", "rar", "tar", "gz", "xz", "bz2", "tgz", "tar.gz", "tar.xz", "tar.bz2",
                "lha", "lzh",
            ],
        )
        .set_title(&t!("ui.select_files_title").to_string())
        .pick_files()?;

    Some(files)
}

async fn extract_archive(file_path: &Path) -> Result<()> {
    if !file_path.exists() {
        return Err(anyhow!(t!(
            "ui.error_file_not_found",
            file = file_path.display()
        )
        .to_string()));
    }

    // 解凍先ディレクトリを決定（ファイルと同じディレクトリ）
    let parent_dir = file_path
        .parent()
        .ok_or_else(|| anyhow!(t!("ui.error_no_parent_dir").to_string()))?;

    let file_stem = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!(t!("ui.error_no_filename").to_string()))?;

    // .tar.gz のような複合拡張子も考慮
    let extract_dir_name = if file_stem.ends_with(".tar") {
        &file_stem[..file_stem.len() - 4]
    } else {
        file_stem
    };

    let mut extract_dir = parent_dir.join(extract_dir_name);

    // 同名ディレクトリが存在する場合、連番をつける
    extract_dir = get_unique_path(extract_dir);

    info!(
        "{}",
        t!(
            "status.extraction_start",
            source = file_path.display(),
            dest = extract_dir.display()
        )
        .to_string()
    );

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
        "lha" => extract_lha(file_path, &extract_dir)?,
        "lzh" => extract_lzh(file_path, &extract_dir)?,
        _ => {
            return Err(anyhow!(t!(
                "ui.error_unsupported_format",
                format = extension
            )
            .to_string()))
        }
    }

    Ok(())
}

fn get_full_extension(path: &Path) -> String {
    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

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
        let file_name = original_path
            .file_name()
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
        .set_title(&t!("ui.error_dialog_title").to_string())
        .set_description(message)
        .set_level(rfd::MessageLevel::Error)
        .show();
}
