use ncurses::*;

#[derive(Clone)]
pub struct Tag {
  content: String,
  window: Option<WINDOW>,
  x: i32,
  y: i32,
  visible: bool
}

pub fn new_tag(content: &str, x: i32, y: i32) -> Tag {
  Tag {
    content: String::from(content),
    window: None,
    x: x,
    y: y,
    visible: true
  }
}

pub fn set_tag_visible(tag: Tag, visible: bool) -> Tag {
  let mut update_tag = tag;
  update_tag.visible = visible;
  update_tag
}

pub fn render_tag(tag: &Tag) {
  let win = match tag.window {
    Some(w) => w,
    None => stdscr(),
  };

  if !tag.visible {
    wrefresh(win);
    return;
  }

  let formatted_content = &format!(" {} ", tag.content);
  wattr_on(win, A_REVERSE());
  mvwaddstr(win, tag.y, tag.x, formatted_content);
  wattr_off(win, A_REVERSE());
}
