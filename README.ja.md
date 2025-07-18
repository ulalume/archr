# Decomp - macOS-like Archive Extractor

macOS likeな解凍ツールです。

## 機能

1. **多様な圧縮形式をサポート**
   - ZIP, 7Z, RAR, TAR, GZ, XZ, BZ2
   - TAR.GZ, TAR.XZ, TAR.BZ2, TGZなどの複合形式
   - ZIPファイルの日本語ファイル名に対応（Shift_JIS/CP932エンコーディング）

2. **簡単な操作**
   - ダブルクリックで起動
   - ファイルダイアログで圧縮ファイルを選択
   - その場所に自動解凍

3. **重複処理**
   - 同名ディレクトリがある場合、自動的に連番をつけてリネーム
   - 例: `archive` → `archive (1)` → `archive (2)`

4. **エラーハンドリング**
   - 解凍失敗時はエラーダイアログを表示
   - ログ出力でデバッグ情報を提供

## 使用方法

### 1. GUIモード（推奨）
```bash
decomp.exe
```
ファイルダイアログが開くので、解凍したい圧縮ファイルを選択してください。

### 2. コマンドラインモード
```bash
decomp.exe archive1.zip archive2.7z archive3.tar.gz
```
指定したファイルを一括で解凍します。

## インストール

```bash
cargo install --git https://github.com/ulalume/decomp
```

### ファイル関連付け（オプション）
Windowsのファイル関連付けで圧縮ファイルのデフォルトアプリに設定できます。  
アプリの場所: `C:\Users\[username]\.cargo\bin\decomp.exe`

## 対応形式

- **ZIP** (.zip)
- **7-Zip** (.7z) 
- **RAR** (.rar)
- **TAR** (.tar)
- **GZIP** (.gz, .tar.gz, .tgz)
- **XZ** (.xz, .tar.xz)
- **BZIP2** (.bz2, .tar.bz2)

## 注意事項

- パスワード付きアーカイブは現在サポートしていません
- 開発中のためテスト不足の可能性があります。重要なファイルはバックアップを取ってからご使用ください
