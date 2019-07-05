use ncurses::*;

pub fn new_app() {
  let locale_conf = LcCategory::all;
  setlocale(locale_conf, "en_US.UTF-8");
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

pub fn get_term_size() -> (i32, i32) {
  let mut x = 0;
  let mut y = 0;
  getmaxyx(stdscr(), &mut y, &mut x);
  (x, y)
}

pub fn refresh_app() {
  refresh();
}

pub fn clear_term() {
  clear();
}