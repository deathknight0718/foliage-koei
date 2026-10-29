//! Extract the SAN14 index and its two known archive variants:
//! cargo run --release -p koei-san14 --example rdb_extract -- <index> <bin> <bin14> <new-output-directory>
//!
//! The output filenames contain file and type KTIDs, not original filenames.
//! Unresolved index entries are listed in unresolved.tsv.
use anyhow::{Context, Result, ensure};
use koei_san14::rdbc::{self, Archive};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    ensure!(args.len() == 4, "usage: rdb_extract <index> <bin> <bin14> <new-output-directory>");
    let index = rdbc::decode_index(&std::fs::read(&args[0]).context("reading RDB index")?)?;
    let mut bin = File::open(&args[1]).context("opening RDB bin archive")?;
    let mut bin14 = File::open(&args[2]).context("opening RDB bin14 archive")?;
    let output = PathBuf::from(&args[3]);
    std::fs::create_dir(&output).context("creating new output directory; an existing directory is never overwritten")?;
    let mut unresolved = BufWriter::new(File::create(output.join("unresolved.tsv"))?);
    writeln!(unresolved, "file_id\ttype_id\tunpacked_size\tflags")?;
    let mut extracted = 0usize;
    let mut missing = 0usize;
    for (number, entry) in index.entries.iter().enumerate() {
        let location = rdbc::parse_location(&entry.location).with_context(|| format!("entry {number} ({:08x}) location", entry.header.file_id))?;
        let Some(location) = location else {
            writeln!(unresolved, "{:08x}\t{:08x}\t{}\t{:08x}", entry.header.file_id, entry.header.type_id, entry.header.unpacked_size, entry.header.flags)?;
            missing += 1;
            continue;
        };
        let archive = match location.archive {
            Archive::Bin => &mut bin,
            Archive::Bin14 => &mut bin14,
        };
        let block = rdbc::read_asset(entry, archive).with_context(|| format!("decoding entry {number} ({:08x}) at {}", entry.header.file_id, entry.location))?;
        let filename = format!("{:08x}.{:08x}.bin", entry.header.file_id, entry.header.type_id);
        let mut file = OpenOptions::new().write(true).create_new(true).open(output.join(&filename)).with_context(|| format!("creating {filename}"))?;
        file.write_all(&block.data).with_context(|| format!("writing {filename}"))?;
        extracted += 1;
    }
    unresolved.flush()?;
    eprintln!("Extracted {extracted} assets; {missing} entries have no location (see unresolved.tsv).");
    Ok(())
}
