use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, default_value_t = String::from("config.json"))]
    /// Path to config
    config: String,
}

fn main() -> anyhow::Result<()> {
    Ok(())
}
