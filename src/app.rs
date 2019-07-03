use ncurses::*;

pub fn new_app() {
  initscr();
  noecho();
  keypad(stdscr(), true);
  refresh();
}

pub fn wait_for_key() -> i32 {
  getch()
}

pub fn translate_key(key: &str) -> i32 {
  match key {
    "arrow_up" => KEY_UP,
    "arrow_down" => KEY_DOWN,
    "arrow_left" => KEY_LEFT,
    "arrow_right" => KEY_RIGHT,
    _ => key.chars().next().unwrap() as i32,
  }
}