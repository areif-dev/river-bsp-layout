use regex::Regex;
use river_layout_toolkit::{GeneratedLayout, Layout, Rectangle};
use std::fmt::Display;

/// Wrapper for errors relating to the creation or operation of a `BSPLayout`
#[non_exhaustive]
#[derive(Debug)]
pub enum BSPLayoutError {
    /// Encountered when a failure occurs in `user_cmd`
    CmdError(String),
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
    pub h_split_perc: f32,

    /// The percentage (between 0.0 and 1.0) of space that should be occupied by the primary window
    /// when a vertical split takes place
    pub v_split_perc: f32,
}

impl BSPLayout {
    /// Initialize a new instance of BSPLayout with inner gaps of 5 pixels and outer gaps of 10
    /// pixels on each side, and a split percent of 50%.
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
            h_split_perc: 0.5,
            v_split_perc: 0.5,
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

    /// Perform the recursive division by two to evenly divide the screen as best
    /// as possible
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
    fn handle_layout_helper(
        &self,
        origin_x: i32,
        origin_y: i32,
        canvas_width: u32,
        canvas_height: u32,
        view_count: u32,
    ) -> GeneratedLayout {
        let mut layout = GeneratedLayout {
            layout_name: "bsp-layout".to_string(),
            views: Vec::with_capacity(view_count as usize),
        };

        // Exit condition. When there is only one window left, it should take up the
        // entire available canvas
        if view_count == 1 {
            layout.views.push(Rectangle {
                x: origin_x,
                y: origin_y,
                width: canvas_width,
                height: canvas_height,
            });

            return layout;
        }

        let half_view_count = view_count / 2;
        let views_remaining = view_count % 2; // In case there are odd number of views

        let h1_width: u32;
        let h1_height: u32;

        let h2_width: u32;
        let h2_height: u32;
        let h2_x: i32;
        let h2_y: i32;

        if canvas_width >= canvas_height {
            /* Vertical Split */

            // In case the width of the area is odd, add one extra pixel if needed
            h1_width = (canvas_width as f32 * self.v_split_perc) as u32 - self.ig_right;
            h1_height = canvas_height;

            h2_width = canvas_width - h1_width - self.ig_left - self.ig_right;
            h2_height = canvas_height;
            h2_x = h1_width as i32 + origin_x + (self.ig_left + self.ig_right) as i32;
            h2_y = origin_y;
        } else {
            /* Horizontal Split */

            h1_width = canvas_width;
            h1_height = (canvas_height as f32 * self.h_split_perc) as u32 - self.ig_bottom;

            h2_width = canvas_width;

            // In case the width of the area is odd, add one extra pixel if needed
            h2_height = canvas_height - h1_height - self.ig_top - self.ig_bottom;
            h2_x = origin_x;
            h2_y = h1_height as i32 + origin_y + (self.ig_bottom + self.ig_top) as i32;
        }

        /* Recursively split the two halves of the window */
        let mut first_half =
            self.handle_layout_helper(origin_x, origin_y, h1_width, h1_height, half_view_count);

        let mut sec_half = self.handle_layout_helper(
            h2_x,
            h2_y,
            h2_width,
            h2_height,
            half_view_count + views_remaining,
        );

        layout.views.append(&mut first_half.views);
        layout.views.append(&mut sec_half.views);

        layout
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
        Ok(i) => i,
        Err(_) => {
            return Err(BSPLayoutError::CmdError(
                "Could not parse f32 from ratio argument".to_string(),
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
        // Outer gap command regex
        let og_re = Regex::new(r"^outer-gap \d+$").unwrap();
        let ogl_re = Regex::new(r"^og-left \d+$").unwrap();
        let ogr_re = Regex::new(r"^og-right \d+$").unwrap();
        let ogb_re = Regex::new(r"^og-bottom \d+$").unwrap();
        let ogt_re = Regex::new(r"^og-top \d+$").unwrap();

        // Inner gap commnad regex
        let inner_re = Regex::new(r"^inner-gap \d+$").unwrap();
        let igl_re = Regex::new(r"^ig-left \d+$").unwrap();
        let igr_re = Regex::new(r"^ig-right \d+$").unwrap();
        let igb_re = Regex::new(r"^ig-bottom \d+$").unwrap();
        let igt_re = Regex::new(r"^ig-top \d+$").unwrap();

        // Split perc command regex
        let default_split_re = Regex::new(r"^split-perc 0*\.\d+$").unwrap();
        let vsr_re = Regex::new(r"^v-split-perc 0*\.\d+$").unwrap();
        let hsr_re = Regex::new(r"^h-split-perc 0*\.\d+$").unwrap();

        if og_re.is_match(&cmd) {
            self.set_all_outer_gaps(parse_gap_cmd(&cmd)?);
        } else if ogl_re.is_match(&cmd) {
            self.og_left = parse_gap_cmd(&cmd)?;
        } else if ogr_re.is_match(&cmd) {
            self.og_right = parse_gap_cmd(&cmd)?;
        } else if ogb_re.is_match(&cmd) {
            self.og_bottom = parse_gap_cmd(&cmd)?;
        } else if ogt_re.is_match(&cmd) {
            self.og_top = parse_gap_cmd(&cmd)?;
        } else if inner_re.is_match(&cmd) {
            self.set_all_inner_gaps(parse_gap_cmd(&cmd)?);
        } else if igl_re.is_match(&cmd) {
            self.ig_left = parse_gap_cmd(&cmd)?;
        } else if igr_re.is_match(&cmd) {
            self.ig_right = parse_gap_cmd(&cmd)?;
        } else if igb_re.is_match(&cmd) {
            self.ig_bottom = parse_gap_cmd(&cmd)?;
        } else if igt_re.is_match(&cmd) {
            self.ig_top = parse_gap_cmd(&cmd)?;
        } else if default_split_re.is_match(&cmd) {
            self.v_split_perc = parse_split_cmd(&cmd)?;
            self.h_split_perc = parse_split_cmd(&cmd)?;
        } else if vsr_re.is_match(&cmd) {
            self.v_split_perc = parse_split_cmd(&cmd)?;
        } else if hsr_re.is_match(&cmd) {
            self.h_split_perc = parse_split_cmd(&cmd)?;
        } else {
            return Err(BSPLayoutError::CmdError(format!(
                "Command not recognized: {}",
                cmd
            )));
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
        let layout = self.handle_layout_helper(
            self.og_left as i32,
            self.og_top as i32,
            usable_width - self.og_left - self.og_right,
            usable_height - self.og_top - self.og_bottom,
            view_count,
        );
        Ok(layout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_layout_helper_one_container() {
        let bsp = BSPLayout::new();
        let layout = bsp.handle_layout_helper(0, 0, 1920, 1080, 1);

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
        let layout = bsp.handle_layout_helper(0, 0, 1920, 1080, 2);

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
        let layout = bsp.handle_layout_helper(0, 0, 1920, 1080, 3);

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
        let layout = bsp.handle_layout_helper(0, 0, 1920, 1080, 4);

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
        bsp.v_split_perc = 0.4;
        bsp.h_split_perc = 0.4;
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

        let second_view = layout.views.get(1).unwrap();
        assert_eq!(
            (
                second_view.x,
                second_view.y,
                second_view.width,
                second_view.height,
            ),
            (0, 432, 768, 648)
        );

        let third_view = layout.views.get(2).unwrap();
        assert_eq!(
            (
                third_view.x,
                third_view.y,
                third_view.width,
                third_view.height,
            ),
            (768, 0, 460, 1080)
        );

        let fourth_view = layout.views.get(3).unwrap();
        assert_eq!(
            (
                fourth_view.x,
                fourth_view.y,
                fourth_view.width,
                fourth_view.height,
            ),
            (1228, 0, 692, 1080)
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
        assert_eq!((bsp.v_split_perc * 10.0).round(), 8.0);
        assert_eq!((bsp.h_split_perc * 10.0).round(), 8.0);

        bsp.user_cmd("h-split-perc 0.4".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!((bsp.v_split_perc * 10.0).round(), 8.0);
        assert_eq!((bsp.h_split_perc * 10.0).round(), 4.0);

        bsp.user_cmd("v-split-perc 0.4".to_string(), None, "eDP-1")
            .unwrap();
        assert_eq!((bsp.v_split_perc * 10.0).round(), 4.0);
        assert_eq!((bsp.h_split_perc * 10.0).round(), 4.0);

        let res = bsp.user_cmd("foo-bar 5678".to_string(), None, "eDP-1");
        assert!(res.is_err());
    }
}
