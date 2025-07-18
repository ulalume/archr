# Decomp - macOS-like Archive Extractor

A macOS-like archive extraction tool.

## Features

1. **Support for Various Archive Formats**
   - ZIP, 7Z, RAR, TAR, GZ, XZ, BZ2
   - Compound formats like TAR.GZ, TAR.XZ, TAR.BZ2, TGZ
   - Supports Japanese filenames in ZIP archives (Shift_JIS/CP932 encoding)

2. **Simple Operation**
   - Launch with double-click
   - Select archive file through file dialog
   - Automatic extraction to the same location

3. **Duplicate Handling**
   - Automatic renaming with sequential numbers when directories with the same name exist
   - Example: `archive` → `archive (1)` → `archive (2)`

4. **Error Handling**
   - Error dialog display when extraction fails
   - Debug information through log output

## Usage

### 1. GUI Mode (Recommended)
```bash
decomp.exe
```
A file dialog will open for you to select the archive file you want to extract.

### 2. Command Line Mode
```bash
decomp.exe archive1.zip archive2.7z archive3.tar.gz
```
Batch extraction of specified files.

## Installation

```bash
cargo install --git https://github.com/ulalume/decomp
```

### File Association (Optional)
You can set this as the default application for archive files in Windows file associations.  
Application location: `C:\Users\[username]\.cargo\bin\decomp.exe`

## Supported Formats

- **ZIP** (.zip)
- **7-Zip** (.7z) 
- **RAR** (.rar)
- **TAR** (.tar)
- **GZIP** (.gz, .tar.gz, .tgz)
- **XZ** (.xz, .tar.xz)
- **BZIP2** (.bz2, .tar.bz2)

## Notes

- Password-protected archives are not currently supported
- This is under development and may have insufficient testing. Please backup important files before use
