use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    recursive: bool,
    pattern: String,
    path: PathBuf,
}

#[derive(Default)]
struct SearchStats {
    dirs: usize,
    files: usize,
    skipped: usize,
}

fn grep_file(file_path: &Path, pattern: &str) -> Result<()> {
    let content = fs::read_to_string(file_path)?;

    for (index, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            println!("{} - {} - {}", file_path.display(), index + 1, line);
        }
    }
    Ok(())
}

fn grep_dir(dir_path: &Path, pattern: &str, recursive: bool) -> Result<SearchStats> {
    let entries = fs::read_dir(dir_path)?;
    let mut stats = SearchStats::default();

    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let path = entry.path();
        if file_type.is_file() {
            match grep_file(&path, pattern) {
                Ok(_) => stats.files += 1,
                Err(_) => stats.skipped += 1,
            }
        } else if recursive && file_type.is_dir() {
            let search_stats = grep_dir(&path, pattern, recursive)?;
            stats.dirs += 1 + search_stats.dirs;
            stats.files += search_stats.files;
            stats.skipped += search_stats.skipped;
        }
    }
    Ok(stats)
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.path.is_dir() {
        let search_stats = grep_dir(&args.path, &args.pattern, args.recursive)?;
        println!(
            "Searched {} directories; {} files; {} skipped",
            search_stats.dirs, search_stats.files, search_stats.skipped
        );
    } else {
        grep_file(&args.path, &args.pattern)?;
    }
    Ok(())
}
