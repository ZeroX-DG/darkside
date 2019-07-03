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