use ncurses::*;

pub struct List {
  items: Vec<String>,
  fill_width: bool,
  window: WINDOW,
  width: i32,
  selected_index: i32,
}

pub fn new_list(x: i32, y: i32, w: i32, h: i32, items: Vec<String>) -> List {
  let win = newwin(h, w, y, x);
  List {
    items: items,
    fill_width: false,
    window: win,
    width: w,
    selected_index: -1,
  }
}

pub fn fill_list_width(list: List, is_fill: bool) -> List {
  List {
    items: list.items,
    fill_width: is_fill,
    window: list.window,
    width: list.width,
    selected_index: list.selected_index,
  }
}

pub fn move_next_list(list: List) -> List {
  let is_in_range = list.selected_index < list.items.len() as i32 - 1;
  let new_index = if is_in_range {
    list.selected_index + 1
  } else {
    list.selected_index
  };
  List {
    items: list.items,
    fill_width: list.fill_width,
    window: list.window,
    width: list.width,
    selected_index: new_index,
  }
}

pub fn move_prev_list(list: List) -> List {
  let is_in_range = list.selected_index > 0;
  let new_index = if is_in_range {
    list.selected_index - 1
  } else {
    list.selected_index
  };
  List {
    items: list.items,
    fill_width: list.fill_width,
    window: list.window,
    width: list.width,
    selected_index: new_index,
  }
}

pub fn render_list(list: &List) {
  let win = list.window;
  box_(win, 0, 0);
  let mut line = 1;
  for item in &list.items {
    if line == list.selected_index + 1 {
      wattr_on(win, A_REVERSE());
      mvwaddstr(win, line, 2, &item);
      wattr_off(win, A_REVERSE());
    } else {
      mvwaddstr(win, line, 2, &item);
    }
    line += 1;
  }
  wrefresh(win);
}