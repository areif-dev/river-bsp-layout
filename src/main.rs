use clap::Parser;
use river_bsp_layout::BSPLayout;
use river_layout_toolkit::run;

/// Layout manager for Wayland tiling compositor River. Creates a grid like Binary Space
/// Partitioned layout where each window is made as equal in size as possible while still
/// occupying all available space in the display
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The size of the gap between adjacent windows. In pixels
    #[arg(short, long, default_value_t = 5)]
    inner_gap: u32,

    /// The default size of the gap between windows and the edge of the screen. In pixels
    #[arg(short, long, default_value_t = 0)]
    default_outer_gap: u32,

    /// The number of pixels to place between the left screen edge and any windows. Overrides
    /// `default_outer_gap` for the left side. Optional.
    #[arg(long, short = 'l')]
    og_left: Option<u32>,

    /// The number of pixels to place between the right screen edge and any windows. Overrides
    /// `default_outer_gap` for the right side. Optional.
    #[arg(long, short = 'r')]
    og_right: Option<u32>,

    /// The number of pixels to place between the bottom screen edge and any windows. Overrides
    /// `default_outer_gap` for the bottom side. Optional.
    #[arg(long, short = 'b')]
    og_bottom: Option<u32>,

    /// The number of pixels to place between the top screen edge and any windows. Overrides
    /// `default_outer_gap` for the top side. Optional.
    #[arg(long, short = 't')]
    og_top: Option<u32>,
}

fn main() {
    let cli = Cli::parse();
    let layout = BSPLayout::new(cli.outer_gap, cli.inner_gap);
    run(layout).unwrap();
}
