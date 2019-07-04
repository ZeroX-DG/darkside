use ncurses::*;
pub mod list;
pub mod tag;
pub mod text;

pub trait Parent {
  fn get_win(&self) -> WINDOW;
}