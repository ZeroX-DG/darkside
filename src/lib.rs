extern crate ncurses;

mod app;
mod widgets;

pub use app::*;
pub use ncurses::constants::*;
pub use widgets::*;

pub const KEY_RETURN: i32 = 10;