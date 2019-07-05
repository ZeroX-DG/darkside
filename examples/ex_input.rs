extern crate darkside;

use darkside::input::*;
use darkside::*;
use std::char;

fn main() {
  new_app();
  let mut active_input = 0;
  let mut input_username = new_input(2, 2, 15, "Username", false);
  let mut input_password = new_input(2, 3, 15, "Password", true);
  loop {
    render_input(&input_username);
    render_input(&input_password);
    let ch = wait_for_key();
    if ch == KEY_DOWN {
      active_input = 1;
    } else if ch == KEY_UP {
      active_input = 0;
    } else if ch == KEY_BACKSPACE {
      if active_input == 0 {
        input_username = remove_input_last_char(input_username);
      } else {
        input_password = remove_input_last_char(input_password);
      }
    } else {
      let input_char = char::from_u32(ch as u32).unwrap();
      if active_input == 0 {
        input_username = add_input_char(input_username, input_char);
      } else {
        input_password = add_input_char(input_password, input_char);
      }
    }
  }
}
