/*
 * MIT License
 *
 * Copyright (c) 2025 Foliage Develop Team
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use anyhow::{Context, Result};
use clap::Parser;
use clap::Subcommand;
use koei_san14::domain::Decoder;
use koei_san14::domain::Encoder;
use koei_san14::domain::fixdataex;
use koei_san14::domain::scedaex;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

// ---------------------------------------------------------------------------

#[derive(Parser)]
#[command(name = "koei-san14")]
#[command(about = "Koei San14 Command Line Interface")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sync {
        #[arg(value_name = "DIRECTORY")]
        directory: PathBuf,
    },
}

// ---------------------------------------------------------------------------

#[rustfmt::skip]
fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Sync { directory } => { sync(directory)?; }
    }
    Ok(())
}

// ---------------------------------------------------------------------------

fn sync(directory: &PathBuf) -> anyhow::Result<()> {
    let entries = fs::read_dir(directory).with_context(|| format!("reading {}", directory.display()))?.map(|entry| entry.with_context(|| format!("reading an entry in {}", directory.display()))).collect::<anyhow::Result<Vec<_>>>()?;
    let source_path = entries.iter().map(|entry| entry.path()).find(|path| path.file_name().is_some_and(|name| name == "fixdataex.s14")).with_context(|| format!("finding fixdataex.s14 in {}", directory.display()))?;
    let bytes = fs::read(&source_path).with_context(|| format!("reading {}", source_path.display()))?;
    let source = fixdataex::FileHex::decode(&bytes).with_context(|| format!("parsing {}", source_path.display()))?;
    let target_pattern = Regex::new(r"^scedaex.*\.s14$")?;
    for entry in entries {
        let path = entry.path();
        if !entry.file_type().with_context(|| format!("reading file type for {}", path.display()))?.is_file() || !target_pattern.is_match(&entry.file_name().to_string_lossy()) {
            continue;
        }
        let bytes = fs::read(&path).with_context(|| format!("reading {}", path.display()))?;
        let mut data = scedaex::FileHex::decode(&bytes).with_context(|| format!("parsing {}", path.display()))?;
        data.tab16.rows = source
            .tab16
            .rows
            .iter()
            .map(|personality| scedaex::Personality {
                name: personality.name, //
                enunciation: personality.enunciation,
                cid1: personality.cid1,
                cid2: personality.cid2,
                cval1: personality.cval1,
                cval2: personality.cval2,
                eid1: personality.eid1,
                eval1: personality.eval1,
                eid2: personality.eid2,
                eval2: personality.eval2,
            })
            .collect();
        fs::write(&path, data.encode()?).with_context(|| format!("writing {}", path.display()))?;
    }
    Ok(())
}
