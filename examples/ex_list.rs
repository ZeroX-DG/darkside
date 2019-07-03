extern crate darkside;

use darkside::list::*;
use darkside::*;

fn main() {
  new_app();
  let mut list = new_list(
    10,
    2,
    20,
    10,
    vec![
      String::from("item 1"),
      String::from("item 2"),
      String::from("item 3"),
      String::from("item 4"),
      String::from("item 5"),
      String::from("item 6"),
      String::from("item 7"),
      String::from("item 8"),
      String::from("item 9"),
      String::from("item 10"),
      String::from("item 11"),
      String::from("item 12"),
      String::from("item 13"),
      String::from("item 14"),
    ],
  );

  list = list_fill_width(list, true);

  loop {
    render_list(&list);
    let ch = wait_for_key();
    let _j_key = translate_key("j");
    let _k_key = translate_key("k");
    let up_key = translate_key("arrow_up");
    let down_key = translate_key("arrow_down");
    if ch == _k_key || ch == up_key {
      list = move_prev_list(list);
    } else if ch == _j_key || ch == down_key {
      list = move_next_list(list);
    }
  }
}