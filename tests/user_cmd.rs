use river_bsp_layout::BSPLayout;
use river_layout_toolkit::Layout;

#[test]
fn test_handle_outer_gaps() {
    let mut bsp = BSPLayout::new();
    bsp.set_all_outer_gaps(0);
    bsp.user_cmd("--outer-gap 5".to_string(), None, "").unwrap();
    assert_eq!(
        (bsp.og_top, bsp.og_left, bsp.og_right, bsp.og_bottom),
        (5, 5, 5, 5)
    );

    bsp.user_cmd("--og-top 10".to_string(), None, "").unwrap();
    assert_eq!(
        (bsp.og_top, bsp.og_left, bsp.og_right, bsp.og_bottom),
        (10, 5, 5, 5)
    );

    bsp.user_cmd("--og-left 10".to_string(), None, "").unwrap();
    assert_eq!(
        (bsp.og_top, bsp.og_left, bsp.og_right, bsp.og_bottom),
        (10, 10, 5, 5)
    );

    bsp.user_cmd("--og-right 10".to_string(), None, "").unwrap();
    assert_eq!(
        (bsp.og_top, bsp.og_left, bsp.og_right, bsp.og_bottom),
        (10, 10, 10, 5)
    );

    bsp.user_cmd("--og-bottom 10".to_string(), None, "")
        .unwrap();
    assert_eq!(
        (bsp.og_top, bsp.og_left, bsp.og_right, bsp.og_bottom),
        (10, 10, 10, 10)
    );

    bsp.user_cmd(
        "--og-top 0 --og-left 1 --og-right 2 --og-bottom 3".to_string(),
        None,
        "",
    )
    .unwrap();
    assert_eq!(
        (bsp.og_top, bsp.og_left, bsp.og_right, bsp.og_bottom),
        (0, 1, 2, 3)
    );
}

#[test]
fn test_handle_inner_gaps() {
    let mut bsp = BSPLayout::new();
    bsp.user_cmd("--inner-gap 5".to_string(), None, "").unwrap();
    bsp.set_all_inner_gaps(0);
    assert_eq!(
        (bsp.ig_top, bsp.ig_left, bsp.ig_right, bsp.ig_bottom),
        (0, 0, 0, 0)
    );

    bsp.user_cmd("--ig-top 10".to_string(), None, "").unwrap();
    assert_eq!(
        (bsp.ig_top, bsp.ig_left, bsp.ig_right, bsp.ig_bottom),
        (10, 0, 0, 0)
    );

    bsp.user_cmd("--ig-left 10".to_string(), None, "").unwrap();
    assert_eq!(
        (bsp.ig_top, bsp.ig_left, bsp.ig_right, bsp.ig_bottom),
        (10, 10, 0, 0)
    );

    bsp.user_cmd("--ig-right 10".to_string(), None, "").unwrap();
    assert_eq!(
        (bsp.ig_top, bsp.ig_left, bsp.ig_right, bsp.ig_bottom),
        (10, 10, 10, 0)
    );

    bsp.user_cmd("--ig-bottom 10".to_string(), None, "")
        .unwrap();
    assert_eq!(
        (bsp.ig_top, bsp.ig_left, bsp.ig_right, bsp.ig_bottom),
        (10, 10, 10, 10)
    );

    bsp.user_cmd(
        "--ig-top 0 --ig-left 1 --ig-right 2 --ig-bottom 3".to_string(),
        None,
        "",
    )
    .unwrap();
    assert_eq!(
        (bsp.ig_top, bsp.ig_left, bsp.ig_right, bsp.ig_bottom),
        (0, 1, 2, 3)
    );
}

#[test]
fn test_handle_start_split() {
    let mut bsp = BSPLayout::new();
    bsp.start_hsplit = false;
    bsp.user_cmd("--start-hsplit".to_string(), None, "")
        .unwrap();
    assert!(bsp.start_hsplit);
    bsp.user_cmd("--start-vsplit".to_string(), None, "")
        .unwrap();
    assert!(!bsp.start_hsplit);

    bsp.user_cmd("--start-vsplit --start-hsplit".to_string(), None, "")
        .unwrap_err();
}

#[test]
fn test_handle_set_split() {
    let mut bsp = BSPLayout::new();
    bsp.vsplit_perc = 0.5;
    bsp.hsplit_perc = 0.5;
    bsp.user_cmd("--split-perc 0.6".to_string(), None, "")
        .unwrap();
    assert_eq!(bsp.vsplit_perc, 0.6);
    assert_eq!(bsp.hsplit_perc, 0.6);

    bsp.user_cmd("--vsplit-perc 0.4".to_string(), None, "")
        .unwrap();
    assert_eq!(bsp.vsplit_perc, 0.4);
    assert_eq!(bsp.hsplit_perc, 0.6);

    bsp.user_cmd("--hsplit-perc 0.3".to_string(), None, "")
        .unwrap();
    assert_eq!(bsp.vsplit_perc, 0.4);
    assert_eq!(bsp.hsplit_perc, 0.3);

    bsp.user_cmd(
        "--split-perc 0.5 --hsplit-perc 0.2 --vsplit-perc 0.1".to_string(),
        None,
        "",
    )
    .unwrap();
    assert_eq!(bsp.vsplit_perc, 0.1);
    assert_eq!(bsp.hsplit_perc, 0.2);
}

#[test]
fn test_handle_ch_split() {
    let mut bsp = BSPLayout::new();
    bsp.vsplit_perc = 0.5;
    bsp.user_cmd("--inc-vsplit 0.3".to_string(), None, "")
        .unwrap();
    assert_eq!(bsp.vsplit_perc, 0.8);

    bsp.user_cmd("--dec-vsplit 0.3".to_string(), None, "")
        .unwrap();
    assert_eq!(bsp.vsplit_perc, 0.5);

    bsp.hsplit_perc = 0.5;
    bsp.user_cmd("--inc-hsplit 0.3".to_string(), None, "")
        .unwrap();
    assert_eq!(bsp.hsplit_perc, 0.8);

    bsp.user_cmd("--dec-hsplit 0.3".to_string(), None, "")
        .unwrap();
    assert_eq!(bsp.hsplit_perc, 0.5);

    bsp.user_cmd("--inc-hsplit 0.3 --inc-vsplit 0.3".to_string(), None, "")
        .unwrap();
    assert_eq!((bsp.hsplit_perc, bsp.vsplit_perc), (0.8, 0.8));
}

#[test]
fn test_handle_reverse() {
    let mut bsp = BSPLayout::new();
    bsp.reversed = false;

    bsp.user_cmd("--reverse".to_string(), None, "").unwrap();
    assert!(bsp.reversed);
    bsp.user_cmd("--reverse".to_string(), None, "").unwrap();
    assert!(!bsp.reversed);
}
