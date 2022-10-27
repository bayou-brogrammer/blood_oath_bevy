use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    pub show_fps: bool,
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
