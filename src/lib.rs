pub mod user_cmd;

use clap::Parser;
use river_layout_toolkit::{GeneratedLayout, Layout, Rectangle};
use std::fmt::Display;

/// Wrapper for errors relating to the creation or operation of a `BSPLayout`
#[non_exhaustive]
#[derive(Debug)]
pub enum BSPLayoutError {
    /// Encountered when a failure occurs in `user_cmd`
    CmdError(String),

    /// Encountered when there a failure occurs when generating a layout
    LayoutError(String),
}

impl Display for BSPLayoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BSPLayoutError {}

/// Create a Binary Space Partitioned layout. Specifically, this layout recursively
/// divides the screen in half. The split will alternate between vertical and horizontal
/// based on which side of the container is longer. This will result in a grid like
/// layout with more-or-less equal sized windows evenly distributed across the screen
pub struct BSPLayout {
    /// Number of pixels to put between the left inside edge of adjacent windows
    pub ig_left: u32,

    /// Number of pixels to put between the right inside edge of adjacent windows
    pub ig_right: u32,

    /// Number of pixels to put between the bottom inside edge of adjacent windows
    pub ig_bottom: u32,

    /// Number of pixels to put between the top inside edge of adjacent windows
    pub ig_top: u32,

    /// Number of pixels to put between the left screen edge and the adjacent windows
    pub og_left: u32,

    /// Number of pixels to put between the right screen edge and the adjacent windows
    pub og_right: u32,

    /// Number of pixels to put between the bottom screen edge and the adjacent windows
    pub og_bottom: u32,

    /// Number of pixels to put between the top screen edge and the adjacent windows
    pub og_top: u32,

    /// The percentage (between 0.0 and 1.0) of space that should be occupied by the primary window
    /// when a horizontal split takes place
    pub hsplit_perc: f32,

    /// The percentage (between 0.0 and 1.0) of space that should be occupied by the primary window
    /// when a vertical split takes place
    pub vsplit_perc: f32,

    /// Whether the first split should be horizontal or not. If true, then start by dividing the
    /// screen in half from right to left. If false, then start by dividing the screen in half from
    /// top to bottom
    pub start_hsplit: bool,

    /// If `true`, new views will be prepended to the list. Otherwise, new views will be appended.
    pub reversed: bool,
}

impl BSPLayout {
    /// Initialize a new instance of BSPLayout with inner gaps of 5 pixels and outer gaps of 10
    /// pixels on each side, a split percent of 50%, and starting on a vertical split
    ///
    /// # Returns
    ///
    /// A new `BSPLayout`
    pub fn new() -> BSPLayout {
        BSPLayout {
            ig_left: 5,
            ig_right: 5,
            ig_bottom: 5,
            ig_top: 5,
            og_left: 10,
            og_right: 10,
            og_top: 10,
            og_bottom: 10,
            hsplit_perc: 0.5,
            vsplit_perc: 0.5,
            reversed: false,
            start_hsplit: false,
        }
    }

    /// Sets all sides of outer gap to `new_gap`
    ///
    /// # Arguments
    ///
    /// * `new_gap` - The value to assign for the gap on all outer edges
    pub fn set_all_outer_gaps(&mut self, new_gap: u32) {
        self.og_top = new_gap;
        self.og_bottom = new_gap;
        self.og_left = new_gap;
        self.og_right = new_gap;
    }

    /// Sets all inner gaps to `new_gap`
    ///
    /// # Arguments
    ///
    /// * `new_gap` - The value to assign for the gap on all inner edges between windows
    pub fn set_all_inner_gaps(&mut self, new_gap: u32) {
        self.ig_top = new_gap;
        self.ig_left = new_gap;
        self.ig_right = new_gap;
        self.ig_bottom = new_gap;
    }

    /// Shared setup between vsplit and hsplit functions. First checks that vsplit_perc and
    /// hsplit_perc are in range, then creates the layout variable, and finally calculates how many
    /// views are in each half of the split
    ///
    /// # Arguments
    ///
    /// * `view_count` - The total number of views accross both splits
    ///
    /// # Returns
    ///
    /// Tuple containing - in order - `half_view_count`, `views_remaining`, and the initial layout
    /// variable
    ///
    /// # Errors
    ///
    /// If either split percentage is not > 0.0 and < 1.0, return `BSPLayoutError`
    fn setup_split(&self, view_count: u32) -> Result<(u32, u32, GeneratedLayout), BSPLayoutError> {
        if self.vsplit_perc <= 0.0
            || self.vsplit_perc >= 1.0
            || self.hsplit_perc <= 0.0
            || self.hsplit_perc >= 1.0
        {
            return Err(BSPLayoutError::LayoutError(
                "Split percents must be > 0.0 and less than 1.0".to_string(),
            ));
        }
        let layout = GeneratedLayout {
            layout_name: "bsp-layout".to_string(),
            views: Vec::with_capacity(view_count as usize),
        };

        let half_view_count = view_count / 2;
        let views_remaining = view_count % 2; // In case there are odd number of views

        Ok((half_view_count, views_remaining, layout))
    }

    /// Divide the screen in two by splitting from right to left first, then subsequently from
    /// top to bottom
    ///
    /// # Arguments
    ///
    /// * `origin_x` - The x position of the top left of the space to be divided
    /// relative to the entire display. For example, if you are dividing the entire
    /// display, then the top left corner is 0, 0. If you are dividing the right
    /// half of a 1920x1080 monitor, then the top left corner would be at 960, 0
    ///
    /// * `origin_y` - The y position of the top left of the space to be divided
    /// relative to the entire display. For example, if you are dividing the entire
    /// display, then the top left corner is 0, 0. If you are dividing the bottom
    /// half of a 1920x1080 monitor, then the top left corner would be at 0, 540
    ///
    /// * `canvas_width` - The width in pixels of the area being divided. If you
    /// are dividing all of a 1920x1080 monitor, then the `canvas_width` would be 1920.
    /// If you are dividing the right half of the monitor, then the width is 960.
    ///
    /// * `canvas_height` - The height in pixels of the area being divided. If you
    /// are dividing all of a 1920x1080 monitor, then the height would be 1080.
    /// If you are dividing the bottom half of the monitor, then the height is 540.
    ///
    /// * `view_count` - How many windows / containers / apps / division the function
    /// needs to make in total.
    ///
    /// # Returns
    ///
    /// A `GeneratedLayout` with `view_count` cells evenly distributed across the screen
    /// in a grid
    fn hsplit(
        &self,
        origin_x: i32,
        origin_y: i32,
        canvas_width: u32,
        canvas_height: u32,
        view_count: u32,
    ) -> Result<GeneratedLayout, BSPLayoutError> {
        let (half_view_count, views_remaining, mut layout) = self.setup_split(view_count)?;

        // Exit condition. When there is only one window left, it should take up the
        // entire available canvas
        if view_count == 1 {
            layout.views.push(Rectangle {
                x: origin_x,
                y: origin_y,
                width: canvas_width,
                height: canvas_height,
            });

            return Ok(layout);
        }

        let mut prime_split = (canvas_height as f32 * self.hsplit_perc) as u32;
        if prime_split == 0 {
            prime_split = 1;
        }
        if prime_split >= canvas_height {
            prime_split = canvas_height - 1;
        }
        let sec_split = canvas_height - prime_split;

        let (prime_sub, sec_sub) = if !self.reversed {
            (self.ig_bottom, self.ig_top)
        } else {
            (self.ig_top, self.ig_bottom)
        };

        let (prime_y, sec_y) = if !self.reversed {
            (origin_y, prime_split as i32 + origin_y + sec_sub as i32)
        } else {
            (sec_split as i32 + origin_y + prime_sub as i32, origin_y)
        };

        let mut prime_layout = self.vsplit(
            origin_x,
            prime_y,
            canvas_width,
            if prime_sub < prime_split {
                prime_split - prime_sub
            } else {
                1
            },
            half_view_count,
        )?;

        let mut sec_layout = self.vsplit(
            origin_x,
            sec_y,
            canvas_width,
            if sec_sub < sec_split {
                sec_split - sec_sub
            } else {
                1
            },
            half_view_count + views_remaining,
        )?;

        layout.views.append(&mut prime_layout.views);
        layout.views.append(&mut sec_layout.views);

        Ok(layout)
    }

    /// Divide the screen in two by splitting from top to bottom first, then subsequently from
    /// right to left
    ///
    /// # Arguments
    ///
    /// * `origin_x` - The x position of the top left of the space to be divided
    /// relative to the entire display. For example, if you are dividing the entire
    /// display, then the top left corner is 0, 0. If you are dividing the right
    /// half of a 1920x1080 monitor, then the top left corner would be at 960, 0
    ///
    /// * `origin_y` - The y position of the top left of the space to be divided
    /// relative to the entire display. For example, if you are dividing the entire
    /// display, then the top left corner is 0, 0. If you are dividing the bottom
    /// half of a 1920x1080 monitor, then the top left corner would be at 0, 540
    ///
    /// * `canvas_width` - The width in pixels of the area being divided. If you
    /// are dividing all of a 1920x1080 monitor, then the `canvas_width` would be 1920.
    /// If you are dividing the right half of the monitor, then the width is 960.
    ///
    /// * `canvas_height` - The height in pixels of the area being divided. If you
    /// are dividing all of a 1920x1080 monitor, then the height would be 1080.
    /// If you are dividing the bottom half of the monitor, then the height is 540.
    ///
    /// * `view_count` - How many windows / containers / apps / division the function
    /// needs to make in total.
    ///
    /// # Returns
    ///
    /// A `GeneratedLayout` with `view_count` cells evenly distributed across the screen
    /// in a grid
    fn vsplit(
        &self,
        origin_x: i32,
        origin_y: i32,
        canvas_width: u32,
        canvas_height: u32,
        view_count: u32,
    ) -> Result<GeneratedLayout, BSPLayoutError> {
        let (half_view_count, views_remaining, mut layout) = self.setup_split(view_count)?;

        // Exit condition. When there is only one window left, it should take up the
        // entire available canvas
        if view_count == 1 {
            layout.views.push(Rectangle {
                x: origin_x,
                y: origin_y,
                width: canvas_width,
                height: canvas_height,
            });

            return Ok(layout);
        }

        let mut prime_split = (canvas_width as f32 * self.vsplit_perc) as u32;
        if prime_split == 0 {
            prime_split = 1;
        }
        if prime_split >= canvas_width {
            prime_split = canvas_width - 1;
        }

        let sec_split = canvas_width - prime_split;

        let (prime_sub, sec_sub) = if !self.reversed {
            (self.ig_right, self.ig_left)
        } else {
            (self.ig_left, self.ig_right)
        };

        let (prime_x, sec_x) = if !self.reversed {
            (origin_x, prime_split as i32 + origin_x + sec_sub as i32)
        } else {
            (sec_split as i32 + origin_x + prime_sub as i32, origin_x)
        };

        let mut prime_layout = self.hsplit(
            prime_x,
            origin_y,
            if prime_sub < prime_split {
                prime_split - prime_sub
            } else {
                1
            },
            canvas_height,
            half_view_count,
        )?;

        let mut sec_layout = self.hsplit(
            sec_x,
            origin_y,
            if sec_sub < sec_split {
                sec_split - sec_sub
            } else {
                1
            },
            canvas_height,
            half_view_count + views_remaining,
        )?;

        layout.views.append(&mut prime_layout.views);
        layout.views.append(&mut sec_layout.views);

        Ok(layout)
    }
}

/// Convenience function for parsing the layout command string and extracting the integer argument
///
/// # Arguments
///
/// * `cmd_str` - The string passed to the user_cmd function that is to be parsed
///
/// # Returns
///
/// If the command is well formed, return the contained gap argument
///
/// # Errors
///
/// If the gap command passed in does not contain an integer argument, returns a `BSPLayoutError`
fn parse_gap_cmd(cmd_str: &str) -> Result<u32, BSPLayoutError> {
    let new_gap_str = match cmd_str.split(" ").last() {
        Some(s) => s,
        None => {
            return Err(BSPLayoutError::CmdError(
                "Gap command missing argument".to_string(),
            ));
        }
    };
    Ok(match new_gap_str.parse::<u32>() {
        Ok(i) => i,
        Err(_) => {
            return Err(BSPLayoutError::CmdError(
                "Could not parse u32 from gap argument".to_string(),
            ));
        }
    })
}

/// Convenience function for parsing the layout command string and extracting the f32 argument
///
/// # Arguments
///
/// * `cmd_str` - The string passed to the user_cmd function that is to be parsed
///
/// # Returns
///
/// If the command is well formed, return the contained ratio argument
///
/// # Errors
///
/// If the split command passed in does not contain a float argument, returns a `BSPLayoutError`
fn parse_split_cmd(cmd_str: &str) -> Result<f32, BSPLayoutError> {
    let new_gap_str = match cmd_str.split(" ").last() {
        Some(s) => s,
        None => {
            return Err(BSPLayoutError::CmdError(
                "Split command missing argument".to_string(),
            ));
        }
    };
    Ok(match new_gap_str.parse::<f32>() {
        Ok(i) => {
            if i <= 0.0 || i >= 1.0 {
                return Err(BSPLayoutError::CmdError(
                    "Split percents must be greater than 0 and less than 1".to_string(),
                ));
            } else {
                i
            }
        }
        Err(_) => {
            return Err(BSPLayoutError::CmdError(
                "Could not parse f32 from percent argument".to_string(),
            ));
        }
    })
}

impl Layout for BSPLayout {
    type Error = BSPLayoutError;

    const NAMESPACE: &'static str = "bsp-layout";

    /// Handle commands passed to the layout with `send-layout-cmd`. Supports individually setting
    /// the gaps on each side of the screen as well as inner edges. Also supports setting all outer
    /// and inner gaps at the same time
    ///
    /// # Examples
    ///
    /// ```
    /// use river_bsp_layout::BSPLayout;
    /// use river_layout_toolkit::Layout;
    ///
    /// // Initialize layout with 0 gaps
    /// let mut bsp = BSPLayout::new();
    /// bsp.set_all_inner_gaps(0);
    /// bsp.set_all_outer_gaps(0);
    ///
    /// // Set gap between windows and the monitor edge to be 5 pixels
    /// let res = bsp.user_cmd("outer-gap 5".to_string(), None, "eDP-1").unwrap();
    /// assert_eq!(bsp.og_top, 5);
    /// assert_eq!(bsp.og_bottom, 5);
    /// assert_eq!(bsp.og_right, 5);
    /// assert_eq!(bsp.og_left, 5);
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `BSPLayoutError::CmdError` if an unrecognized command is passed
    /// or if an invalid argument is passed to a valid command.
    fn user_cmd(
        &mut self,
        cmd: String,
        _tags: Option<u32>,
        _output: &str,
    ) -> Result<(), Self::Error> {
        let mut cmd: Vec<&str> = cmd.split(" ").collect();
        cmd.insert(0, "");
        let cmd = match user_cmd::UserCmd::try_parse_from(cmd) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                return Ok(());
            }
        };
        if cmd.start_hsplit && cmd.start_vsplit {
            return Err(BSPLayoutError::CmdError(
                "start-hsplit and start-vsplit are mutually exclusive. Please select only one"
                    .to_string(),
            ));
        } else if cmd.start_hsplit && !cmd.start_vsplit {
            self.start_hsplit = true;
        } else if cmd.start_vsplit && !cmd.start_hsplit {
            self.start_hsplit = false;
        }

        if cmd.reverse {
            self.reversed = !self.reversed;
        }

        if let Some(p) = cmd.default_split_perc {
            self.hsplit_perc = p;
            self.vsplit_perc = p;
        }
        if let Some(p) = cmd.vsplit_perc {
            self.vsplit_perc = p;
        }
        if let Some(p) = cmd.hsplit_perc {
            self.hsplit_perc = p;
        }

        if let Some(g) = cmd.default_outer_gap {
            self.og_top = g;
            self.og_bottom = g;
            self.og_right = g;
            self.og_left = g;
        }
        if let Some(g) = cmd.og_top {
            self.og_top = g;
        }
        if let Some(g) = cmd.og_bottom {
            self.og_bottom = g;
        }
        if let Some(g) = cmd.og_right {
            self.og_right = g;
        }
        if let Some(g) = cmd.og_left {
            self.og_left = g;
        }

        if let Some(g) = cmd.default_inner_gap {
            self.ig_top = g;
            self.ig_bottom = g;
            self.ig_right = g;
            self.ig_left = g;
        }
        if let Some(g) = cmd.ig_top {
            self.ig_top = g;
        }
        if let Some(g) = cmd.ig_bottom {
            self.ig_bottom = g;
        }
        if let Some(g) = cmd.ig_right {
            self.ig_right = g;
        }
        if let Some(g) = cmd.ig_left {
            self.ig_left = g;
        }

        if let Some(p) = cmd.inc_hsplit {
            if self.hsplit_perc + p < 1.0 {
                self.hsplit_perc += p;
            } else {
                self.hsplit_perc = 0.9999
            }
        }
        if let Some(p) = cmd.inc_vsplit {
            if self.vsplit_perc + p < 1.0 {
                self.vsplit_perc += p;
            } else {
                self.vsplit_perc = 0.9999;
            }
        }

        if let Some(p) = cmd.dec_hsplit {
            if self.hsplit_perc - p > 0.0 {
                self.hsplit_perc -= p;
            } else {
                self.hsplit_perc = 0.0001
            }
        }
        if let Some(p) = cmd.dec_vsplit {
            if self.vsplit_perc - p > 0.0 {
                self.vsplit_perc -= p;
            } else {
                self.vsplit_perc = 0.0001
            }
        }

        Ok(())
    }

    /// Create the geometry for the `BSPLayout`
    ///
    /// # Arguments
    ///
    /// * `view_count` - The number of views / windows / containers to divide the screen into
    /// * `usable_width` - How many pixels wide the whole display is
    /// * `usable_height` - How many pixels tall the whole display is
    /// * `_tags` - Int representing which tags are currently active based on which
    /// bit is toggled
    /// * `_output` - The name of the output to generate the layout on
    ///
    /// # Examples
    ///
    /// ```
    /// use river_bsp_layout::BSPLayout;
    /// use river_layout_toolkit::Layout;
    ///
    /// let mut bsp = BSPLayout::new();
    /// bsp.generate_layout(2, 1920, 1080, 0b000000001, "eDP-1").unwrap();
    /// ```
    fn generate_layout(
        &mut self,
        view_count: u32,
        usable_width: u32,
        usable_height: u32,
        _tags: u32,
        _output: &str,
    ) -> Result<GeneratedLayout, Self::Error> {
        if !self.start_hsplit {
            Ok(self.vsplit(
                self.og_left as i32,
                self.og_top as i32,
                usable_width - self.og_left - self.og_right,
                usable_height - self.og_top - self.og_bottom,
                view_count,
            ))?
        } else {
            Ok(self.hsplit(
                self.og_left as i32,
                self.og_top as i32,
                usable_width - self.og_left - self.og_right,
                usable_height - self.og_top - self.og_bottom,
                view_count,
            ))?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_layout_helper_one_container() {
        let mut bsp = BSPLayout::new();
        bsp.set_all_outer_gaps(0);
        bsp.set_all_inner_gaps(0);
        let layout = bsp.generate_layout(1, 1920, 1080, 1, "").unwrap();

        assert_eq!(layout.views.len(), 1);
        let first_view = layout.views.get(0).unwrap();
        assert_eq!(
            (
                first_view.x,
                first_view.y,
                first_view.width,
                first_view.height
            ),
            (0, 0, 1920, 1080)
        );
    }

    #[test]
    fn test_handle_layout_helper_two_containers() {
        let mut bsp = BSPLayout::new();
        bsp.set_all_outer_gaps(0);
        bsp.set_all_inner_gaps(0);
        let layout = bsp.generate_layout(2, 1920, 1080, 1, "").unwrap();

        assert_eq!(layout.views.len(), 2);
        let first_view = layout.views.get(0).unwrap();
        assert_eq!(
            (
                first_view.x,
                first_view.y,
                first_view.width,
                first_view.height
            ),
            (0, 0, 960, 1080)
        );

        let second_view = layout.views.get(1).unwrap();
        assert_eq!(
            (
                second_view.x,
                second_view.y,
                second_view.width,
                second_view.height
            ),
            (960, 0, 960, 1080)
        );
    }

    #[test]
    fn test_handle_layout_helper_three_containers() {
        let mut bsp = BSPLayout::new();
        bsp.set_all_outer_gaps(0);
        bsp.set_all_inner_gaps(0);
        let layout = bsp.generate_layout(3, 1920, 1080, 1, "").unwrap();

        assert_eq!(layout.views.len(), 3);
        let first_view = layout.views.get(0).unwrap();
        assert_eq!(
            (
                first_view.x,
                first_view.y,
                first_view.width,
                first_view.height
            ),
            (0, 0, 960, 1080)
        );

        let second_view = layout.views.get(1).unwrap();
        assert_eq!(
            (
                second_view.x,
                second_view.y,
                second_view.width,
                second_view.height
            ),
            (960, 0, 960, 540)
        );

        let third_view = layout.views.get(2).unwrap();
        assert_eq!(
            (
                third_view.x,
                third_view.y,
                third_view.width,
                third_view.height
            ),
            (960, 540, 960, 540)
        );
    }

    #[test]
    fn test_handle_layout_helper_four_containers() {
        let mut bsp = BSPLayout::new();
        bsp.set_all_outer_gaps(0);
        bsp.set_all_inner_gaps(0);
        let layout = bsp.generate_layout(4, 1920, 1080, 1, "").unwrap();

        assert_eq!(layout.views.len(), 4);
        let first_view = layout.views.get(0).unwrap();
        assert_eq!(
            (
                first_view.x,
                first_view.y,
                first_view.width,
                first_view.height
            ),
            (0, 0, 960, 540)
        );

        let second_view = layout.views.get(1).unwrap();
        assert_eq!(
            (
                second_view.x,
                second_view.y,
                second_view.width,
                second_view.height
            ),
            (0, 540, 960, 540)
        );

        let third_view = layout.views.get(2).unwrap();
        assert_eq!(
            (
                third_view.x,
                third_view.y,
                third_view.width,
                third_view.height
            ),
            (960, 0, 960, 540)
        );

        let fourth_view = layout.views.get(3).unwrap();
        assert_eq!(
            (
                fourth_view.x,
                fourth_view.y,
                fourth_view.width,
                fourth_view.height
            ),
            (960, 540, 960, 540)
        );
    }

    #[test]
    fn test_generate_layout_no_gaps() {
        let mut bsp = BSPLayout::new();
        bsp.set_all_inner_gaps(0);
        bsp.set_all_outer_gaps(0);
        let layout = bsp.generate_layout(4, 1920, 1080, 1, "eDP-1").unwrap();

        assert_eq!(layout.views.len(), 4);
        let first_view = layout.views.get(0).unwrap();
        assert_eq!(
            (
                first_view.x,
                first_view.y,
                first_view.width,
                first_view.height
            ),
            (0, 0, 960, 540)
        );

        let second_view = layout.views.get(1).unwrap();
        assert_eq!(
            (
                second_view.x,
                second_view.y,
                second_view.width,
                second_view.height
            ),
            (0, 540, 960, 540)
        );

        let third_view = layout.views.get(2).unwrap();
        assert_eq!(
            (
                third_view.x,
                third_view.y,
                third_view.width,
                third_view.height
            ),
            (960, 0, 960, 540)
        );

        let fourth_view = layout.views.get(3).unwrap();
        assert_eq!(
            (
                fourth_view.x,
                fourth_view.y,
                fourth_view.width,
                fourth_view.height
            ),
            (960, 540, 960, 540)
        );
    }

    #[test]
    fn test_generate_layout_with_gaps() {
        let mut bsp = BSPLayout::new();
        bsp.set_all_outer_gaps(10);
        bsp.og_top = 0;
        bsp.set_all_inner_gaps(10);
        let layout = bsp.generate_layout(4, 1920, 1080, 1, "eDP-1").unwrap();

        assert_eq!(layout.views.len(), 4);
        let first_view = layout.views.get(0).unwrap();
        assert_eq!(
            (
                first_view.x,
                first_view.y,
                first_view.width,
                first_view.height
            ),
            (10, 0, 940, 525)
        );

        let second_view = layout.views.get(1).unwrap();
        assert_eq!(
            (
                second_view.x,
                second_view.y,
                second_view.width,
                second_view.height
            ),
            (10, 545, 940, 525)
        );

        let third_view = layout.views.get(2).unwrap();
        assert_eq!(
            (
                third_view.x,
                third_view.y,
                third_view.width,
                third_view.height
            ),
            (970, 0, 940, 525)
        );

        let fourth_view = layout.views.get(3).unwrap();
        assert_eq!(
            (
                fourth_view.x,
                fourth_view.y,
                fourth_view.width,
                fourth_view.height
            ),
            (970, 545, 940, 525)
        );
    }

    #[test]
    fn test_generate_layout_split() {
        let mut bsp = BSPLayout::new();
        bsp.vsplit_perc = 0.0;
        assert!(bsp.generate_layout(4, 1920, 1080, 1, "eDP-1").is_err());

        bsp.vsplit_perc = 0.4;
        bsp.hsplit_perc = 0.4;
        bsp.set_all_outer_gaps(0);
        bsp.set_all_inner_gaps(0);
        let layout = bsp.generate_layout(4, 1920, 1080, 1, "eDP-1").unwrap();

        let first_view = layout.views.get(0).unwrap();
        assert_eq!(
            (
                first_view.x,
                first_view.y,
                first_view.width,
                first_view.height,
            ),
            (0, 0, 768, 432)
        );

        let second_view = layout.views.get(2).unwrap();
        assert_eq!(
            (
                second_view.x,
                second_view.y,
                second_view.width,
                second_view.height,
            ),
            (768, 0, 1152, 432)
        );

        let third_view = layout.views.get(1).unwrap();
        assert_eq!(
            (
                third_view.x,
                third_view.y,
                third_view.width,
                third_view.height,
            ),
            (0, 432, 768, 648)
        );

        let fourth_view = layout.views.get(3).unwrap();
        assert_eq!(
            (
                fourth_view.x,
                fourth_view.y,
                fourth_view.width,
                fourth_view.height,
            ),
            (768, 432, 1152, 648)
        );
    }

    #[test]
    fn test_generate_layout_reverse() {
        let mut bsp = BSPLayout::new();
        bsp.set_all_inner_gaps(0);
        bsp.set_all_outer_gaps(0);
        bsp.reversed = true;
        let layout = bsp.generate_layout(3, 1920, 1080, 1, "eDP-1").unwrap();

        assert_eq!(layout.views.len(), 3);
        let first_view = layout.views.get(0).unwrap();
        assert_eq!(
            (
                first_view.x,
                first_view.y,
                first_view.width,
                first_view.height
            ),
            (960, 0, 960, 1080)
        );

        let second_view = layout.views.get(1).unwrap();
        assert_eq!(
            (
                second_view.x,
                second_view.y,
                second_view.width,
                second_view.height
            ),
            (0, 540, 960, 540)
        );

        let third_view = layout.views.get(2).unwrap();
        assert_eq!(
            (
                third_view.x,
                third_view.y,
                third_view.width,
                third_view.height
            ),
            (0, 0, 960, 540)
        );
    }

    #[test]
    fn test_send_user_cmds() {
        let mut bsp = BSPLayout::new();
        bsp.user_cmd("inner-gap 0".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!(bsp.ig_top, 0);
        assert_eq!(bsp.ig_bottom, 0);
        assert_eq!(bsp.ig_left, 0);
        assert_eq!(bsp.ig_right, 0);

        bsp.user_cmd("ig-left 1".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!(bsp.ig_left, 1);

        bsp.user_cmd("ig-right 1".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!(bsp.ig_right, 1);

        bsp.user_cmd("ig-top 1".to_string(), None, "eDP-1").unwrap();
        assert_eq!(bsp.ig_top, 1);

        bsp.user_cmd("ig-bottom 1".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!(bsp.ig_bottom, 1);

        bsp.user_cmd("outer-gap 10".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!(bsp.og_top, 10);
        assert_eq!(bsp.og_left, 10);
        assert_eq!(bsp.og_right, 10);
        assert_eq!(bsp.og_bottom, 10);

        bsp.user_cmd("og-left 1".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!(bsp.og_left, 1);

        bsp.user_cmd("og-right 1".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!(bsp.og_right, 1);

        bsp.user_cmd("og-top 1".to_string(), None, "eDP-1").unwrap();
        assert_eq!(bsp.og_top, 1);

        bsp.user_cmd("og-bottom 1".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!(bsp.og_bottom, 1);

        bsp.user_cmd("split-perc 0.8".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!((bsp.vsplit_perc * 10.0).round(), 8.0);
        assert_eq!((bsp.hsplit_perc * 10.0).round(), 8.0);

        bsp.user_cmd("hsplit-perc 0.4".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!((bsp.vsplit_perc * 10.0).round(), 8.0);
        assert_eq!((bsp.hsplit_perc * 10.0).round(), 4.0);

        bsp.user_cmd("vsplit-perc 0.4".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!((bsp.vsplit_perc * 10.0).round(), 4.0);
        assert_eq!((bsp.hsplit_perc * 10.0).round(), 4.0);

        assert!(bsp
            .user_cmd("split-perc 0.0".to_string(), None, "eDP-1")
            .is_err());
        assert!(bsp
            .user_cmd("split-perc 1.0".to_string(), None, "eDP-1")
            .is_err());
        assert!(bsp
            .user_cmd("split-perc 1.1".to_string(), None, "eDP-1")
            .is_err());
        assert!(bsp
            .user_cmd("split-perc -0.1".to_string(), None, "eDP-1")
            .is_err());

        bsp.user_cmd("split-perc 0.5".to_string(), None, "eDP-1")
            .unwrap();
        assert!(bsp
            .user_cmd("inc-vsplit-perc 0.5".to_string(), None, "eDP-1")
            .is_err());
        assert!(bsp
            .user_cmd("dec-vsplit-perc 0.5".to_string(), None, "eDP-1")
            .is_err());
        assert!(bsp
            .user_cmd("inc-hsplit-perc 0.5".to_string(), None, "eDP-1")
            .is_err());
        assert!(bsp
            .user_cmd("dec-hsplit-perc 0.5".to_string(), None, "eDP-1")
            .is_err());
        bsp.user_cmd("inc-vsplit-perc 0.4".to_string(), None, "eDP-1")
            .unwrap();
        bsp.user_cmd("dec-vsplit-perc 0.4".to_string(), None, "eDP-1")
            .unwrap();
        bsp.user_cmd("inc-hsplit-perc 0.4".to_string(), None, "eDP-1")
            .unwrap();
        bsp.user_cmd("dec-hsplit-perc 0.4".to_string(), None, "eDP-1")
            .unwrap();

        bsp.reversed = false;
        bsp.user_cmd("reverse".to_string(), None, "eDP-1").unwrap();
        assert!(bsp.reversed);
        bsp.user_cmd("reverse".to_string(), None, "eDP-1").unwrap();
        assert!(!bsp.reversed);

        let res = bsp.user_cmd("foo-bar 5678".to_string(), None, "eDP-1");
        assert!(res.is_err());
    }
}
