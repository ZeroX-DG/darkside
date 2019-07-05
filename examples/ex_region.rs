extern crate darkside;

use darkside::region::*;
use darkside::list::*;
use darkside::*;

fn main() {
    new_app();
    let mut str_vec = vec![];
    for i in 1..21 {
        str_vec.push(format!("item {}", i));
    }
    let region = new_region(1, 1, 12, 10, Some("Title"), Border::All);
    let mut list = new_list(2, 2, 10, 8, str_vec);
    list = set_list_fill_width(list, true);
    loop {
        render_region(&region);
        render_list(&list);
        let ch = wait_for_key();
        if ch == KEY_UP {
            list = move_prev_list_item(list);
        } else if ch == KEY_DOWN {
            list = move_next_list_item(list);
        }
    }
}
