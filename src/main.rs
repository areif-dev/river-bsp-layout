use clap::Parser;
use river_bsp_layout::BSPLayout;
use river_layout_toolkit::run;

/// Layout manager for Wayland tiling compositor River. Creates a grid like Binary Space
/// Partitioned layout where each window is made as equal in size as possible while still
/// occupying all available space in the display
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The number of pixels to pad each inner edge of a window by default.
    #[arg(short = 'i', long = "inner-gap", default_value_t = 0)]
    default_inner_gap: u32,

    /// The number of pixels to pad the left inner edge of each window. This Overrides
    /// `default_inner_gap`. Optional
    #[arg(long, short = 'l')]
    ig_left: Option<u32>,

    /// The number of pixels to pad the right inner edge of each window. This Overrides
    /// `default_inner_gap`. Optional
    #[arg(long, short = 'r')]
    ig_right: Option<u32>,

    /// The number of pixels to pad the bottom inner edge of each window. This Overrides
    /// `default_inner_gap`. Optional
    #[arg(long, short = 'b')]
    ig_bottom: Option<u32>,

    /// The number of pixels to pad the top inner edge of each window. This Overrides
    /// `default_inner_gap`. Optional
    #[arg(long, short = 't')]
    ig_top: Option<u32>,

    /// The default size of the gap between windows and the edge of the screen.
    #[arg(short = 'o', long = "outer-gap", default_value_t = 0)]
    default_outer_gap: u32,

    /// The number of pixels to place between the left screen edge and any windows. Overrides
    /// `default_outer_gap` for the left side. Optional.
    #[arg(long, short = 'L')]
    og_left: Option<u32>,

    /// The number of pixels to place between the right screen edge and any windows. Overrides
    /// `default_outer_gap` for the right side. Optional.
    #[arg(long, short = 'R')]
    og_right: Option<u32>,

    /// The number of pixels to place between the bottom screen edge and any windows. Overrides
    /// `default_outer_gap` for the bottom side. Optional.
    #[arg(long, short = 'B')]
    og_bottom: Option<u32>,

    /// The number of pixels to place between the top screen edge and any windows. Overrides
    /// `default_outer_gap` for the top side. Optional.
    #[arg(long, short = 'T')]
    og_top: Option<u32>,

    /// The percentage of the available split area that is to be retained by the parent.
    #[arg(long, short, default_value_t = 0.5)]
    split_ratio: f32,
}

fn main() {
    let cli = Cli::parse();
    let mut layout = BSPLayout::new();
    layout.ig_left = cli.ig_left.unwrap_or(cli.default_inner_gap);
    layout.ig_right = cli.ig_right.unwrap_or(cli.default_inner_gap);
    layout.ig_bottom = cli.ig_bottom.unwrap_or(cli.default_inner_gap);
    layout.ig_top = cli.ig_top.unwrap_or(cli.default_inner_gap);

    layout.og_left = cli.og_left.unwrap_or(cli.default_outer_gap);
    layout.og_right = cli.og_right.unwrap_or(cli.default_outer_gap);
    layout.og_bottom = cli.og_bottom.unwrap_or(cli.default_outer_gap);
    layout.og_top = cli.og_top.unwrap_or(cli.default_outer_gap);

    layout.split_ratio = cli.split_ratio;

    run(layout).unwrap();
}
