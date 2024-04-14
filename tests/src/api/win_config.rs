use nvim_oxi as oxi;
use nvim_oxi::api::{self, types::*, Buffer, Window};

#[oxi::test]
fn open_win_empty_config() {
    let buf = Buffer::current();
    let config = WindowConfig::builder().build();
    let res = api::open_win(&buf, false, &config);
    assert!(
        res.is_err(),
        "config is missing required fields `relative`, `height` and `width`"
    );
}

#[oxi::test]
fn open_win_basic_config() {
    let buf = api::create_buf(true, true).unwrap();
    let config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .height(10)
        .width(5)
        .row(1.5)
        .col(1.5)
        .build();

    let res = api::open_win(&buf, false, &config);
    assert!(res.is_ok(), "{res:?}");

    let win = res.unwrap();

    let got = win.get_config();
    assert!(got.is_ok(), "{got:?}");

    let got = got.unwrap();
    assert_eq!(config.relative, got.relative);
    assert_eq!(config.height, got.height);
    assert_eq!(config.width, got.width);
    assert_eq!(config.row, got.row);
    assert_eq!(config.col, got.col);
}

#[oxi::test]
fn open_win_full_config() {
    let buf = api::create_buf(true, true).unwrap();

    let config = WindowConfig::builder()
        .relative(WindowRelativeTo::Window(Window::current()))
        .anchor(WindowAnchor::SouthWest)
        .height(10)
        .width(5)
        .bufpos(7, 5)
        .row(1.5)
        .col(1.5)
        .focusable(false)
        .external(false)
        .zindex(300u32)
        .style(WindowStyle::Minimal)
        .border(WindowBorder::from((
            None, None, None, '>', None, None, None, '<',
        )))
        .build();

    let res = api::open_win(&buf, false, &config);
    assert!(res.is_ok(), "{res:?}");

    let win = res.unwrap();

    let got = win.get_config();
    assert!(got.is_ok(), "{got:?}");

    let got = got.unwrap();
    assert_eq!(config.relative, got.relative);
    assert_eq!(config.height, got.height);
    assert_eq!(config.width, got.width);
    assert_eq!(config.row, got.row);
    assert_eq!(config.col, got.col);
    assert_eq!(config.border, got.border);
}

#[oxi::test]
fn set_config() {
    let buf = api::create_buf(true, true).unwrap();

    let initial = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .height(10)
        .width(5)
        .row(1.5)
        .col(1.5)
        .build();

    let mut win = api::open_win(&buf, false, &initial).unwrap();

    let config = WindowConfig::builder()
        .relative(WindowRelativeTo::Window(Window::current()))
        .anchor(WindowAnchor::SouthWest)
        .height(10)
        .width(5)
        .bufpos(7, 5)
        .row(1.5)
        .col(1.5)
        .focusable(false)
        .external(false)
        .zindex(300)
        .style(WindowStyle::Minimal)
        .border(WindowBorder::from((
            None, None, None, '>', None, None, None, '<',
        )))
        .build();

    assert_eq!(Ok(()), win.set_config(&config));
}
