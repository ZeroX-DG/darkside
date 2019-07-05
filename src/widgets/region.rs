use super::Border;
use ncurses::*;

pub struct Region<'a> {
    window: WINDOW,
    title: Option<&'a str>,
    border: Border
}

pub fn new_region<'a>(x: i32, y: i32, w: i32, h: i32, title: Option<&'a str>, border: Border) -> Region<'a> {
    let window = newwin(h, w, y, x);
    Region {
        window: window,
        title: title,
        border: border
    }
}

pub fn set_region_title<'a>(region: Region<'a>, title: &'a str) -> Region<'a> {
    let mut update_region = region;
    update_region.title = Some(title);
    update_region
}

pub fn render_region(region: &Region) {
    wclear(region.window);
    match &region.border {
        Border::All => box_(region.window, 0, 0),
        Border::Left => wborder(region.window, 0, 32, 32, 32, 32, 32, 32, 32),
        Border::Right => wborder(region.window, 32, 0, 32, 32, 32, 32, 32, 32),
        Border::Top => wborder(region.window, 32, 32, 0, 32, 32, 32, 32, 32),
        Border::Bottom => wborder(region.window, 32, 32, 32, 0, 32, 32, 32, 32)
    };
    if let Some(title) = region.title {
        wattr_on(region.window, A_BOLD());
        mvwaddstr(region.window, 0, 1, &title);
        wattr_off(region.window, A_BOLD());
    }
    wrefresh(region.window);
}

