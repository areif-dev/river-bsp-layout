use clap::Parser;
use regex::Regex;
use river_layout_toolkit::{run, GeneratedLayout, Layout, Rectangle};
use std::convert::Infallible;

/// Layout manager for Wayland tiling compositor River. Creates a grid like Binary Space
/// Partitioned layout where each window is made as equal in size as possible while still
/// occupying all available space in the display
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The size of the gap between adjacent windows. In pixels
    #[arg(short, long, default_value_t = 5)]
    inner_gap: u32,

    /// The size of the gap between windows and the edge of the screen. In pixels
    #[arg(short, long, default_value_t = 5)]
    outer_gap: u32,
}

/// Create a Binary Space Partitioned layout. Specifically, this layout recursively
/// divides the screen in half. The split will alternate between vertical and horizontal
/// based on which side of the container is longer. This will result in a grid like
/// layout with more-or-less equal sized windows even distributed across the screen
///
/// 3 Window Example:
/// +---------------+---------------+
/// |               |               |
/// |               |               |
/// |               |               |
/// +               +---------------+
/// |               |               |
/// |               |               |
/// |               |               |
/// +---------------+---------------+
///
/// 4 Window Example:
/// +---------------+---------------+
/// |               |               |
/// |               |               |
/// |               |               |
/// +---------------+---------------+
/// |               |               |
/// |               |               |
/// |               |               |
/// +---------------+---------------+
struct BSPLayout {
    outer_gap: u32,
    inner_gap: u32,
}

impl BSPLayout {
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
            h1_width =
                canvas_width / 2 + canvas_width % 2 - self.inner_gap / 2 - self.inner_gap % 2;
            h1_height = canvas_height;

            h2_width = canvas_width / 2 - self.inner_gap / 2;
            h2_height = canvas_height;
            h2_x = h1_width as i32 + origin_x + self.inner_gap as i32;
            h2_y = origin_y;
        } else {
            /* Horizontal Split */

            h1_width = canvas_width;
            h1_height =
                canvas_height / 2 + canvas_height % 2 - self.inner_gap / 2 - self.inner_gap % 2;

            h2_width = canvas_width;

            // In case the width of the area is odd, add one extra pixel if needed
            h2_height = canvas_height / 2 - self.inner_gap / 2;
            h2_x = origin_x;
            h2_y = h1_height as i32 + origin_y + self.inner_gap as i32;
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

impl Layout for BSPLayout {
    type Error = Infallible;

    const NAMESPACE: &'static str = "bsp-layout";

    /// Handle commands passed to the layout with `send-layout-cmd`
    fn user_cmd(
        &mut self,
        _cmd: String,
        _tags: Option<u32>,
        _output: &str,
    ) -> Result<(), Self::Error> {
        let outer_re = Regex::new(r"^outer-gap \d+$").unwrap();
        let inner_re = Regex::new(r"^inner-gap \d+$").unwrap();

        if outer_re.is_match(&_cmd) {
            let new_gap_str = _cmd.split(" ").last().unwrap();
            let new_gap = new_gap_str.parse::<u32>().unwrap();

            self.outer_gap = new_gap;
        } else if inner_re.is_match(&_cmd) {
            let new_gap_str = _cmd.split(" ").last().unwrap();
            let new_gap = new_gap_str.parse::<u32>().unwrap();

            self.inner_gap = new_gap;
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
    fn generate_layout(
        &mut self,
        view_count: u32,
        usable_width: u32,
        usable_height: u32,
        _tags: u32,
        _output: &str,
    ) -> Result<GeneratedLayout, Self::Error> {
        let layout = self.handle_layout_helper(
            self.outer_gap as i32,
            self.outer_gap as i32,
            usable_width - self.outer_gap * 2,
            usable_height - self.outer_gap * 2,
            view_count,
        );
        Ok(layout)
    }
}

fn main() {
    let cli = Cli::parse();
    let layout = BSPLayout {
        outer_gap: cli.outer_gap,
        inner_gap: cli.inner_gap,
    };
    run(layout).unwrap();
}
