use anyhow::Result;
use clap::Parser;


#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    recursive: bool,
    pattern: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    
    Ok(())
}
