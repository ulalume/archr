use encoding_rs::SHIFT_JIS;
use std::path::PathBuf;

/// ファイル名のデコード（日本語対応）
/// 生のバイト配列からファイル名を適切にデコードする
pub fn decode_filename(raw_bytes: &[u8]) -> String {
    // まず、UTF-8として有効かチェック
    if let Ok(utf8_str) = std::str::from_utf8(raw_bytes) {
        // すでに正しくデコードされている場合
        if !utf8_str
            .chars()
            .any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t')
        {
            return utf8_str.to_string();
        }
    }

    // UTF-8でない場合、Shift_JIS (CP932) としてデコードを試行
    let (decoded, _, had_errors) = SHIFT_JIS.decode(raw_bytes);
    if !had_errors {
        decoded.to_string()
    } else {
        // Shift_JISでもデコードに失敗した場合、代替文字を使用
        String::from_utf8_lossy(raw_bytes).to_string()
    }
}

/// ファイル名をデコードしてPathBufとして返す
pub fn decode_filename_as_pathbuf(raw_bytes: &[u8]) -> PathBuf {
    PathBuf::from(decode_filename(raw_bytes))
}
