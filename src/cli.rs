use clap::Parser;
use std::path::PathBuf;

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
        value_parser = clap::value_parser!(i32).range(5..),
        default_value = "60",
        help = "Size of a single key unit in pixels"
    )]
    pub size: i32,

    #[arg(
        long,
        value_enum,
        default_value = "bottom-right",
        help = "Position of the overlay window"
    )]
    pub position: WindowPosition,

    #[arg(
        long,
        value_parser = clap::value_parser!(u64).range(0..),
        default_value = "2000",
        help = "Timeout for the overlay in milliseconds"
    )]
    pub timeout: u64,

    #[arg(long, default_value = "10", help = "Margin around the overlay window")]
    pub margin: u32,
}
