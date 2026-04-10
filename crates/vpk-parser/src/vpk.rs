use std::fs;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VpkError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid VPK signature: expected 0x55AA1234, got 0x{0:08X}")]
    InvalidSignature(u32),
    #[error("Unsupported VPK version: {0}")]
    UnsupportedVersion(u32),
    #[error("File not found in VPK: {0}")]
    FileNotFound(String),
    #[error("Invalid directory tree")]
    InvalidTree,
}

const VPK_SIGNATURE: u32 = 0x55AA1234;
const VPK_VERSION_1: u32 = 1;
const VPK_VERSION_2: u32 = 2;
const DIR_ARCHIVE_INDEX: u16 = 0x7FFF;

#[derive(Debug)]
pub struct VpkHeader {
    pub signature: u32,
    pub version: u32,
    pub tree_size: u32,
}

#[derive(Debug, Clone)]
pub struct VpkEntry {
    pub path: String,
    pub crc: u32,
    pub preload_bytes: u16,
    pub archive_index: u16,
    pub entry_offset: u32,
    pub entry_length: u32,
    pub preload_data: Vec<u8>,
}

/// Read a null-terminated string from a reader
fn read_null_terminated_string<R: Read>(reader: &mut R) -> Result<String, VpkError> {
    let mut bytes = Vec::new();
    let mut byte = [0u8; 1];
    loop {
        reader.read_exact(&mut byte)?;
        if byte[0] == 0 {
            break;
        }
        bytes.push(byte[0]);
    }
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

fn read_u32<R: Read>(reader: &mut R) -> Result<u32, VpkError> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u16<R: Read>(reader: &mut R) -> Result<u16, VpkError> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

/// Parse the VPK header
pub fn read_header<R: Read>(reader: &mut R) -> Result<VpkHeader, VpkError> {
    let signature = read_u32(reader)?;
    if signature != VPK_SIGNATURE {
        return Err(VpkError::InvalidSignature(signature));
    }

    let version = read_u32(reader)?;
    if version != VPK_VERSION_1 && version != VPK_VERSION_2 {
        return Err(VpkError::UnsupportedVersion(version));
    }

    let tree_size = read_u32(reader)?;

    if version == VPK_VERSION_2 {
        // Skip v2-specific header fields
        let mut skip = [0u8; 16];
        reader.read_exact(&mut skip)?;
    }

    Ok(VpkHeader {
        signature,
        version,
        tree_size,
    })
}

/// Parse the directory tree and return all entries
pub fn read_directory<R: Read>(reader: &mut R) -> Result<Vec<VpkEntry>, VpkError> {
    let mut entries = Vec::new();

    loop {
        let extension = read_null_terminated_string(reader)?;
        if extension.is_empty() {
            break;
        }

        loop {
            let dir_path = read_null_terminated_string(reader)?;
            if dir_path.is_empty() {
                break;
            }

            loop {
                let filename = read_null_terminated_string(reader)?;
                if filename.is_empty() {
                    break;
                }

                let crc = read_u32(reader)?;
                let preload_bytes = read_u16(reader)?;
                let archive_index = read_u16(reader)?;
                let entry_offset = read_u32(reader)?;
                let entry_length = read_u32(reader)?;

                let _terminator = read_u16(reader)?;

                let mut preload_data = vec![0u8; preload_bytes as usize];
                if preload_bytes > 0 {
                    reader.read_exact(&mut preload_data)?;
                }

                let full_path = if dir_path == " " {
                    format!("{filename}.{extension}")
                } else {
                    format!("{dir_path}/{filename}.{extension}")
                };

                entries.push(VpkEntry {
                    path: full_path,
                    crc,
                    preload_bytes,
                    archive_index,
                    entry_offset,
                    entry_length,
                    preload_data,
                });
            }
        }
    }

    Ok(entries)
}

/// Extract a specific file from a VPK directory file
pub fn extract_file(vpk_dir_path: &Path, target_path: &str) -> Result<Vec<u8>, VpkError> {
    let mut file = fs::File::open(vpk_dir_path)?;
    let header = read_header(&mut file)?;

    let tree_start = if header.version == VPK_VERSION_1 {
        12u64 // signature(4) + version(4) + tree_size(4)
    } else {
        28u64 // + 16 bytes of v2 header
    };

    file.seek(SeekFrom::Start(tree_start))?;
    let entries = read_directory(&mut file)?;

    let entry = entries
        .iter()
        .find(|e| e.path == target_path)
        .ok_or_else(|| VpkError::FileNotFound(target_path.to_string()))?;

    let mut data = entry.preload_data.clone();

    if entry.entry_length > 0 {
        if entry.archive_index == DIR_ARCHIVE_INDEX {
            let data_offset = tree_start + header.tree_size as u64 + entry.entry_offset as u64;
            file.seek(SeekFrom::Start(data_offset))?;
            let mut buf = vec![0u8; entry.entry_length as usize];
            file.read_exact(&mut buf)?;
            data.extend(buf);
        } else {
            let dir_str = vpk_dir_path
                .to_str()
                .ok_or(VpkError::InvalidTree)?;
            let base = dir_str.trim_end_matches("_dir.vpk");
            let archive_path = format!("{base}_{:03}.vpk", entry.archive_index);
            let mut archive = fs::File::open(&archive_path)?;
            archive.seek(SeekFrom::Start(entry.entry_offset as u64))?;
            let mut buf = vec![0u8; entry.entry_length as usize];
            archive.read_exact(&mut buf)?;
            data.extend(buf);
        }
    }

    Ok(data)
}

/// Extract items_game.txt from a Dota 2 VPK
pub fn extract_items_game(vpk_path: &Path) -> Result<String, VpkError> {
    let data = extract_file(vpk_path, "scripts/items/items_game.txt")?;
    Ok(String::from_utf8_lossy(&data).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vpk_signature_constant() {
        assert_eq!(VPK_SIGNATURE, 0x55AA1234);
    }

    #[test]
    fn test_read_header_invalid_signature() {
        let data: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut cursor = io::Cursor::new(data);
        let result = read_header(&mut cursor);
        assert!(result.is_err());
        match result.unwrap_err() {
            VpkError::InvalidSignature(sig) => assert_eq!(sig, 0),
            other => panic!("Expected InvalidSignature, got: {other}"),
        }
    }

    #[test]
    fn test_read_header_v1() {
        let mut data = Vec::new();
        data.extend(&VPK_SIGNATURE.to_le_bytes());
        data.extend(&1u32.to_le_bytes()); // version
        data.extend(&100u32.to_le_bytes()); // tree_size

        let mut cursor = io::Cursor::new(data);
        let header = read_header(&mut cursor).unwrap();
        assert_eq!(header.signature, VPK_SIGNATURE);
        assert_eq!(header.version, 1);
        assert_eq!(header.tree_size, 100);
    }

    #[test]
    fn test_read_null_terminated_string() {
        let data = b"hello\0world\0";
        let mut cursor = io::Cursor::new(data.to_vec());
        assert_eq!(read_null_terminated_string(&mut cursor).unwrap(), "hello");
        assert_eq!(read_null_terminated_string(&mut cursor).unwrap(), "world");
    }

    #[test]
    fn test_read_empty_string() {
        let data = b"\0";
        let mut cursor = io::Cursor::new(data.to_vec());
        assert_eq!(read_null_terminated_string(&mut cursor).unwrap(), "");
    }
}
