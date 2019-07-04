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

pub fn translate_key(key: char) -> i32 {
  key as i32
}