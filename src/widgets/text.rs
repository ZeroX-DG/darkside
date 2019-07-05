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
  Highlighted
}

/// Create new text widget
pub fn new_text(text: &str, x: i32, y: i32) -> Text {
  Text {
    data: String::from(text),
    effects: vec![],
    window: None,
    x: x,
    y: y,
  }
}

/// Set the text effect to display for text widget
pub fn set_text_effects(text: Text, effects: Vec<TextEffect>) -> Text {
  let mut update_text = text;
  update_text.effects = effects;
  update_text
}

/// Center the text base on its' parent
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

  let mut update_text = text;
  update_text.x = new_x;
  update_text.y = new_y;
  update_text
}

fn translate_text_effect(effect: &TextEffect) -> attr_t {
  match effect {
    TextEffect::Bold => A_BOLD(),
    TextEffect::Italic => A_ITALIC(),
    TextEffect::Underline => A_UNDERLINE(),
    TextEffect::Normal => A_NORMAL(),
    TextEffect::Highlighted => A_REVERSE(),
  }
}

/// Render the text widget
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
