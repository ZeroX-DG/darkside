use ncurses::*;

pub fn new_app() {
  initscr();
  noecho();
  keypad(stdscr(), true);
}

pub fn pause_app() {
  getch();
}