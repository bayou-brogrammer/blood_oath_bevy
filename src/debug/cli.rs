use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct VoxCli {
    /// Turn debugging information on
    #[arg(short, long)]
    pub show_fps: bool,
}

pub fn parse_cli() -> VoxCli {
    VoxCli::parse()
}
