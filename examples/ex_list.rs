extern crate darkside;

use darkside::list::*;
use darkside::text::*;
use darkside::*;

fn main() {
  new_app();
  let mut str_vec = vec![];
  let mut current_list = 1;
  for i in 1..21 {
    str_vec.push(format!("item {}", i));
  }
  let mut list_1 = new_list(10, 2, 20, 10, str_vec);
  let item_vec = vec![
    String::from("This is long"),
    String::from("This is longer"),
    String::from("This is even longer"),
    String::from("This is the longest one"),
  ];
  let mut list_2 = new_list(30, 2, 20, 10, item_vec);

  let mut instruction = new_text("Use <- to focus left list and -> for right list", 10, 12);
  instruction = set_text_effects(instruction, vec![TextEffect::Bold]);

  list_1 = set_list_fill_width(list_1, true);
  list_2 = set_list_fill_width(list_2, true);
  list_2 = set_list_text_overflow(list_2, TextOverflow::Ellipsis);
  list_2 = set_list_title(list_2, "List");

  let _j_key = translate_key("j");
  let _k_key = translate_key("k");
  let up_key = translate_key("arrow_up");
  let down_key = translate_key("arrow_down");
  let left_key = translate_key("arrow_left");
  let right_key = translate_key("arrow_right");

  loop {
    render_list(&list_1);
    render_list(&list_2);
    render_text(&instruction);
    let ch = wait_for_key();
    if ch == _k_key || ch == up_key {
      if current_list == 1 {
        list_1 = move_prev_list_item(list_1);
      } else {
        list_2 = move_prev_list_item(list_2);
      }
    } else if ch == _j_key || ch == down_key {
      if current_list == 1 {
        list_1 = move_next_list_item(list_1);
      } else {
        list_2 = move_next_list_item(list_2);
      }
    } else if ch == left_key {
      current_list = 1;
    } else if ch == right_key {
      current_list = 2;
    }
  }
}