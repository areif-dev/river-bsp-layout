use clap::{ArgGroup, Parser};

use crate::BSPLayout;

#[derive(Parser)]
pub struct UserCmd {
    /// The number of pixels to pad each inner edge of a window by default.
    #[arg(short = 'i', long = "inner-gap", help_heading = "Inner Gap Options")]
    pub default_inner_gap: Option<u32>,

    /// The number of pixels to pad the left inner edge of each window. This Overrides
    /// `default_inner_gap`. Optional
    #[arg(long, short = 'l', help_heading = "Inner Gap Options")]
    pub ig_left: Option<u32>,

    /// The number of pixels to pad the right inner edge of each window. This Overrides
    /// `default_inner_gap`. Optional
    #[arg(long, short = 'r', help_heading = "Inner Gap Options")]
    pub ig_right: Option<u32>,

    /// The number of pixels to pad the bottom inner edge of each window. This Overrides
    /// `default_inner_gap`. Optional
    #[arg(long, short = 'b', help_heading = "Inner Gap Options")]
    pub ig_bottom: Option<u32>,

    /// The number of pixels to pad the top inner edge of each window. This Overrides
    /// `default_inner_gap`. Optional
    #[arg(long, short = 't', help_heading = "Inner Gap Options")]
    pub ig_top: Option<u32>,

    /// The default size of the gap between windows and the edge of the screen.
    #[arg(short = 'o', long = "outer-gap", help_heading = "Outer Gap Options")]
    pub default_outer_gap: Option<u32>,

    /// The number of pixels to place between the left screen edge and any windows. Overrides
    /// `default_outer_gap` for the left side. Optional.
    #[arg(long, short = 'L', help_heading = "Outer Gap Options")]
    pub og_left: Option<u32>,

    /// The number of pixels to place between the right screen edge and any windows. Overrides
    /// `default_outer_gap` for the right side. Optional.
    #[arg(long, short = 'R', help_heading = "Outer Gap Options")]
    pub og_right: Option<u32>,

    /// The number of pixels to place between the bottom screen edge and any windows. Overrides
    /// `default_outer_gap` for the bottom side. Optional.
    #[arg(long, short = 'B', help_heading = "Outer Gap Options")]
    pub og_bottom: Option<u32>,

    /// The number of pixels to place between the top screen edge and any windows. Overrides
    /// `default_outer_gap` for the top side. Optional.
    #[arg(long, short = 'T', help_heading = "Outer Gap Options")]
    pub og_top: Option<u32>,

    /// The default percentage of available area that the primary window should occupy after any
    /// split takes place.
    #[arg(long = "split-perc", short = 's', help_heading = "Split Options")]
    pub default_split_perc: Option<f32>,

    /// The percentage of available area that the primary window should occupy after a horizontal
    /// split. This will override the value of `default_split_perc` only for horizontal splits.
    #[arg(long, short = 'H', help_heading = "Split Options")]
    pub hsplit_perc: Option<f32>,

    /// The percentage of available area that the primary window should occupy after a vertical
    /// split. This will override the value of `default_split_perc` only for vertical splits.
    #[arg(long, short, help_heading = "Split Options")]
    pub vsplit_perc: Option<f32>,

    /// Set the first split to horizontal
    #[arg(long, help_heading = "Split Options")]
    pub start_hsplit: bool,

    /// Set the first split to vertical
    #[arg(long, help_heading = "Split Options", conflicts_with("start_hsplit"))]
    pub start_vsplit: bool,

    /// Increase the hsplit percentage by a certain amount.
    #[arg(long, help_heading = "Split Options")]
    pub inc_hsplit: Option<f32>,

    /// Increase the vsplit percentage by a certain amount.
    #[arg(long, help_heading = "Split Options")]
    pub inc_vsplit: Option<f32>,

    /// Decrease the vsplit percentage by a certain amount.
    #[arg(long, help_heading = "Split Options")]
    pub dec_vsplit: Option<f32>,

    /// Decrease the hsplit percentage by a certain amount.
    #[arg(long, help_heading = "Split Options")]
    pub dec_hsplit: Option<f32>,

    /// Reverse the order of the views as well as the order they are added.
    #[arg(long, help_heading = "Other Options")]
    pub reverse: bool,
}
