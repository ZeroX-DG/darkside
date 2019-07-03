use ncurses::*;

pub struct Text {
  data: String,
  effects: Vec<TextEffect>,
  window: Option<WINDOW>,
  x: i32,
  y: i32,
}

pub enum TextEffect {
  Bold,
  Italic,
  Underline,
  Normal,
}

pub fn new_text(text: &str, x: i32, y: i32) -> Text {
  Text {
    data: String::from(text),
    effects: vec![],
    window: None,
    x: x,
    y: y,
  }
}

pub fn set_text_effects(text: Text, effects: Vec<TextEffect>) -> Text {
  Text {
    data: text.data,
    effects: effects,
    window: text.window,
    x: text.x,
    y: text.y,
  }
}

pub fn center_text(text: Text, center_h: bool, center_v: bool) -> Text {
  let win = match text.window {
    Some(w) => w,
    None => stdscr(),
  };

  let new_x = if center_h {
    let win_w = getmaxx(win);
    (win_w - text.data.chars().count() as i32) / 2
  } else {
    text.x
  };

  let new_y = if center_v {
    let win_h = getmaxy(win);
    (win_h - text.data.lines().count() as i32) / 2
  } else {
    text.y
  };

  Text {
    data: text.data,
    effects: text.effects,
    window: text.window,
    x: new_x,
    y: new_y,
  }
}

fn translate_text_effect(effect: &TextEffect) -> attr_t {
  match effect {
    TextEffect::Bold => A_BOLD(),
    TextEffect::Italic => A_ITALIC(),
    TextEffect::Underline => A_UNDERLINE(),
    TextEffect::Normal => A_NORMAL(),
  }
}

pub fn render_text(text: &Text) {
  let win = match text.window {
    Some(w) => w,
    None => stdscr(),
  };
  for effect in &text.effects {
    let a_effect = translate_text_effect(effect);
    wattr_on(win, a_effect);
  }
  mvwaddstr(win, text.y, text.x, &text.data);
  for effect in &text.effects {
    let a_effect = translate_text_effect(effect);
    wattr_off(win, a_effect);
  }
}