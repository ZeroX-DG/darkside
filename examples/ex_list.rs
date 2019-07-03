extern crate darkside;

use darkside::list::*;
use darkside::*;

fn main() {
  new_app();
  let mut list = new_list(10, 2, 20, 10, vec![
    String::from("Hello"),
    String::from("World")
  ]);

  loop {
    render_list(&list);
    let ch = wait_for_key();
    let _j_key = 'j' as i32;
    let _k_key = 'k' as i32;
    if ch == _k_key {
      list = move_prev_list(list);
    } else if ch == _j_key {
      list = move_next_list(list);
    }
  }
}