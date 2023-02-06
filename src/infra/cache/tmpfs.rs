use chrono::{DateTime, Utc};
use parking_lot::RwLock;
#[cfg(not(target_os = "windows"))]
use rsfs::*;
#[cfg(not(target_os = "windows"))]
use rsfs::{DirEntry, Metadata};
#[cfg(not(target_os = "windows"))]
use std::io::{Read, Write};
use std::path::Path;

pub struct TmpDirEntry {
    pub location: String,
    pub last_modified: DateTime<Utc>,
    pub size: usize,
}

#[cfg(not(target_os = "windows"))]
lazy_static! {
    pub static ref FS: RwLock<mem::FS> = RwLock::new(mem::FS::new());
}

#[inline(always)]
#[cfg(not(target_os = "windows"))]
pub fn write_file<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<(), std::io::Error> {
    let mut f = FS.write().create_file(path)?;
    f.write_all(data)?;
    Ok(())
}

#[inline(always)]
#[cfg(not(target_os = "windows"))]
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, std::io::Error> {
    let mut file = FS.write().open_file(path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

#[inline(always)]
#[cfg(not(target_os = "windows"))]
pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    FS.write().create_dir(path)
}

#[inline(always)]
#[cfg(not(target_os = "windows"))]
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    FS.write().create_dir_all(path)
}

#[inline(always)]
#[cfg(not(target_os = "windows"))]
pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<Vec<TmpDirEntry>, std::io::Error> {
    let mut values = Vec::with_capacity(2);
    let files = FS.read().read_dir(path)?;
    for file in files {
        let file = file.unwrap();
        let file_path = file.path();
        let file_metadata = file.metadata().unwrap();
        values.push(TmpDirEntry {
            location: file_path.to_str().unwrap().to_string(),
            last_modified: file_metadata.modified().unwrap().into(),
            size: file_metadata.len() as usize,
        });
    }
    Ok(values)
}

#[inline(always)]
#[cfg(not(target_os = "windows"))]
pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    FS.write().remove_dir(path)
}

#[inline(always)]
#[cfg(not(target_os = "windows"))]
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    FS.write().remove_dir_all(path)
}

#[inline(always)]
#[cfg(not(target_os = "windows"))]
pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    FS.write().remove_file(path)
}

#[inline(always)]
#[cfg(target_os = "windows")]
pub fn write_file<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<(), std::io::Error> {
    crate::common::file::put_file_contents(path.as_ref().to_str().unwrap(), data)
}

#[inline(always)]
#[cfg(target_os = "windows")]
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, std::io::Error> {
    crate::common::file::get_file_contents(path.as_ref().to_str().unwrap())
}

#[inline(always)]
#[cfg(target_os = "windows")]
pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    std::fs::create_dir(path)
}

#[inline(always)]
#[cfg(target_os = "windows")]
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(path)
}

#[inline(always)]
#[cfg(target_os = "windows")]
pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<Vec<TmpDirEntry>, std::io::Error> {
    let mut values = Vec::with_capacity(2);
    let files = std::fs::read_dir(path)?;
    for file in files {
        let file = file.unwrap();
        let file_path = file.path();
        let file_metadata = file.metadata().unwrap();
        values.push(TmpDirEntry {
            location: file_path.to_str().unwrap().to_string(),
            last_modified: file_metadata.modified().unwrap().into(),
            size: file_metadata.len() as usize,
        });
    }
    Ok(values)
}

#[inline(always)]
#[cfg(target_os = "windows")]
pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    std::fs::remove_dir(path)
}

#[inline(always)]
#[cfg(target_os = "windows")]
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    std::fs::remove_dir_all(path)
}

#[inline(always)]
#[cfg(target_os = "windows")]
pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    std::fs::remove_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_read_file() {
        let path = "/test.txt";
        let _ = write_file(path, b"hello world").unwrap();
        let data = read_file(path).unwrap();
        assert_eq!(data, b"hello world");
        remove_file(path).unwrap();
    }

    #[test]
    fn create_read_directory() {
        let path = "/test_dir/abc/";
        assert_eq!(true, create_dir_all(path).is_ok());
        assert_eq!(false, remove_dir("/test_dir/").is_ok());
        assert_eq!(true, remove_dir_all("/test_dir/").is_ok());
    }
}
