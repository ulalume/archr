# Decomp - macOS-like Archive Extractor

🇯🇵 日本語版 README は[こちら](./README.ja.md)をご覧ください。

A macOS-like archive extraction tool. Cross-platform, but primarily targeting Windows.

## Installation

```bash
cargo install --git https://github.com/ulalume/decomp
```

## Usage

### 1. File Association (Recommended)
Set this as the default application for archive files in Windows file associations. Once configured, you can simply double-click any archive file to extract it in place.
Application location: `C:\Users\[username]\.cargo\bin\decomp.exe`

### 2. GUI Mode
```bash
decomp.exe
```
A file dialog will open for you to select the archive file you want to extract.

### 3. Command Line Mode
```bash
decomp.exe archive1.zip archive2.7z archive3.tar.gz
```
Batch extraction of specified files.

## Supported Formats

- **ZIP** (.zip)
- **7-Zip** (.7z)
- **RAR** (.rar)
- **TAR** (.tar)
- **GZIP** (.gz, .tar.gz, .tgz)
- **XZ** (.xz, .tar.xz)
- **BZIP2** (.bz2, .tar.bz2)
- **LHA/LZH** (.lha, .lzh)
- Supports Japanese filenames (Shift_JIS/CP932 encoding)

## Notes

- Password-protected archives are not currently supported
