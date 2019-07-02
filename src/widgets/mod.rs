use ncurses::*;
pub mod text;

pub trait Parent {
  fn get_win(&self) -> WINDOW;
}