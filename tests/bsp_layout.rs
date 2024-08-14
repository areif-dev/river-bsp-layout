use river_bsp_layout::BSPLayout;
use river_layout_toolkit::Layout;

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
