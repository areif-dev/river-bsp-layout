use clap::Parser;

use crate::{BSPLayout, BSPLayoutError};

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
    #[arg(long, help_heading = "Split Options")]
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

impl UserCmd {
    pub fn handle_outer_gaps(&self, layout: &mut BSPLayout) {
        if let Some(g) = self.default_outer_gap {
            layout.og_top = g;
            layout.og_bottom = g;
            layout.og_right = g;
            layout.og_left = g;
        }
        if let Some(g) = self.og_top {
            layout.og_top = g;
        }
        if let Some(g) = self.og_bottom {
            layout.og_bottom = g;
        }
        if let Some(g) = self.og_right {
            layout.og_right = g;
        }
        if let Some(g) = self.og_left {
            layout.og_left = g;
        }
    }

    pub fn handle_inner_gaps(&self, layout: &mut BSPLayout) {
        if let Some(g) = self.default_inner_gap {
            layout.ig_top = g;
            layout.ig_bottom = g;
            layout.ig_right = g;
            layout.ig_left = g;
        }
        if let Some(g) = self.ig_top {
            layout.ig_top = g;
        }
        if let Some(g) = self.ig_bottom {
            layout.ig_bottom = g;
        }
        if let Some(g) = self.ig_right {
            layout.ig_right = g;
        }
        if let Some(g) = self.ig_left {
            layout.ig_left = g;
        }
    }

    pub fn handle_ch_split(&self, layout: &mut BSPLayout) {
        if let Some(p) = self.inc_hsplit {
            if layout.hsplit_perc + p < 1.0 {
                layout.hsplit_perc += p;
            } else {
                layout.hsplit_perc = 0.9999
            }
        }
        if let Some(p) = self.inc_vsplit {
            if layout.vsplit_perc + p < 1.0 {
                layout.vsplit_perc += p;
            } else {
                layout.vsplit_perc = 0.9999;
            }
        }

        if let Some(p) = self.dec_hsplit {
            if layout.hsplit_perc - p > 0.0 {
                layout.hsplit_perc -= p;
            } else {
                layout.hsplit_perc = 0.0001
            }
        }
        if let Some(p) = self.dec_vsplit {
            if layout.vsplit_perc - p > 0.0 {
                layout.vsplit_perc -= p;
            } else {
                layout.vsplit_perc = 0.0001
            }
        }
    }

    pub fn handle_start_split(&self, layout: &mut BSPLayout) -> Result<(), BSPLayoutError> {
        if self.start_hsplit && self.start_vsplit {
            eprintln!(
                "start-hsplit and start-vsplit are mutually exclusive. Please select only one"
            );
            return Err(BSPLayoutError::CmdError(
                "start-hsplit and start-vsplit are mutually exclusive. Please select only one"
                    .to_string(),
            ));
        } else if self.start_hsplit && !self.start_vsplit {
            layout.start_hsplit = true;
        } else if self.start_vsplit && !self.start_hsplit {
            layout.start_hsplit = false;
        }

        Ok(())
    }

    pub fn handle_set_split(&self, layout: &mut BSPLayout) {
        if let Some(p) = self.default_split_perc {
            layout.hsplit_perc = p;
            layout.vsplit_perc = p;
        }
        if let Some(p) = self.vsplit_perc {
            layout.vsplit_perc = p;
        }
        if let Some(p) = self.hsplit_perc {
            layout.hsplit_perc = p;
        }
    }

    pub fn handle_reverse(&self, layout: &mut BSPLayout) {
        if self.reverse {
            layout.reversed = !layout.reversed;
        }
    }
}
