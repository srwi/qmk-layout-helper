use clap::Parser;
use std::path::PathBuf;

fn parse_size(s: &str) -> Result<(u32, u32), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("Size must be in the format 'width,height'".to_string());
    }
    let width: u32 = parts[0].parse().map_err(|_| "Invalid width")?;
    let height: u32 = parts[1].parse().map_err(|_| "Invalid height")?;
    Ok((width, height))
}

#[derive(clap::ValueEnum, Clone)]
pub enum WindowPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Bottom,
    Top,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(help = "Path to the keyboard information JSON file")]
    pub keyboard_config: PathBuf,

    #[arg(
        short = 'l',
        long = "layout",
        default_value = "LAYOUT",
        help = "Name of the keyboard layout to display"
    )]
    pub layout_name: String,

    #[arg(
        long,
        value_parser = parse_size,
        default_value = "700,240",
        help = "Size of the overlay window in the format 'width,height'"
    )]
    pub size: (u32, u32),

    #[arg(
        long,
        value_enum,
        default_value = "bottom-right",
        help = "Position of the overlay window"
    )]
    pub position: WindowPosition,

    #[arg(
        long,
        value_parser = clap::value_parser!(u64).range(100..),
        default_value = "5000",
        help = "Timeout for the overlay in milliseconds"
    )]
    pub timeout: u64,

    #[arg(long, default_value = "10", help = "Margin around the overlay window")]
    pub margin: u32,
}
