use anyhow::{Context, bail};
use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

const BUFFER_SIZE: usize = 8 * 1024 * 1024;

#[test]
#[ignore = "scans a large local game directory"]
fn test_scan() -> anyhow::Result<()> {
    scan(b"\x1e\x6d\xdf\x5b", Path::new("D:\\Projects\\foliage-koei\\.data\\system.rdb.decoded"))
}

fn scan(flag: &[u8], path: &Path) -> anyhow::Result<()> {
    if flag.is_empty() {
        bail!("the binary flag cannot be empty");
    }
    if !path.is_dir() {
        bail!("scan directory does not exist or is not a directory: {}", path.display());
    }
    let mut failure_table = vec![0; flag.len()];
    let mut prefix_len = 0;
    for index in 1..flag.len() {
        while prefix_len > 0 && flag[prefix_len] != flag[index] {
            prefix_len = failure_table[prefix_len - 1];
        }
        if flag[prefix_len] == flag[index] {
            prefix_len += 1;
        }
        failure_table[index] = prefix_len;
    }
    let mut directories = vec![path.to_path_buf()];
    let mut matches: Vec<PathBuf> = Vec::new();
    let mut buffer = vec![0; BUFFER_SIZE];
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(&directory).with_context(|| format!("reading directory {}", directory.display()))? {
            let entry = entry.with_context(|| format!("reading entry in {}", directory.display()))?;
            let file_path = entry.path();
            let file_type = entry.file_type().with_context(|| format!("reading file type for {}", file_path.display()))?;
            if file_type.is_dir() {
                directories.push(file_path);
                continue;
            }
            if !file_type.is_file() {
                continue;
            }
            let file = File::open(&file_path).with_context(|| format!("opening {}", file_path.display()))?;
            let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
            let mut matched = 0;
            'read_file: loop {
                let bytes_read = reader.read(&mut buffer).with_context(|| format!("reading {}", file_path.display()))?;
                if bytes_read == 0 {
                    break;
                }
                for &byte in &buffer[..bytes_read] {
                    while matched > 0 && flag[matched] != byte {
                        matched = failure_table[matched - 1];
                    }
                    if flag[matched] == byte {
                        matched += 1;
                        if matched == flag.len() {
                            matches.push(file_path);
                            break 'read_file;
                        }
                    }
                }
            }
        }
    }
    matches.sort_unstable();
    println!("Matched {} file(s):", matches.len());
    for path in matches {
        println!("{}", path.display());
    }
    Ok(())
}
