use ncurses::*;

pub struct Tag {
  content: String,
  window: Option<WINDOW>,
  x: i32,
  y: i32,
}

pub fn new_tag(content: &str, x: i32, y: i32) -> Tag {
  Tag {
    content: String::from(content),
    window: None,
    x: x,
    y: y,
  }
}

pub fn render_tag(tag: &Tag) {
  let win = match tag.window {
    Some(w) => w,
    None => stdscr(),
  };

  let formatted_content = &format!(" {} ", tag.content);
  wattr_on(win, A_REVERSE());
  mvwaddstr(win, tag.y, tag.x, formatted_content);
  wattr_off(win, A_REVERSE());
}