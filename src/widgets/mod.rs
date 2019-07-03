use ncurses::*;
pub mod text;
pub mod list;

pub trait Parent {
  fn get_win(&self) -> WINDOW;
}