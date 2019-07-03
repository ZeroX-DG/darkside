use ncurses::*;

pub struct List {
  items: Vec<String>,
  fill_width: bool,
  window: WINDOW,
  inner_window: WINDOW,
  width: i32,
  height: i32,
  selected_index: i32,
  scroll_top: i32,
  text_overflow: TextOverflow,
}

pub enum TextOverflow {
  Ellipsis,
  Hidden,
}

pub fn new_list(x: i32, y: i32, w: i32, h: i32, items: Vec<String>) -> List {
  let win = newwin(h, w, y, x);
  let inner_window = newwin(h - 2, w - 2, y + 1, x + 1);
  List {
    items: items,
    fill_width: false,
    window: win,
    inner_window: inner_window,
    width: w,
    height: h,
    selected_index: -1,
    scroll_top: 0,
    text_overflow: TextOverflow::Ellipsis,
  }
}

pub fn list_text_overflow(list: List, overflow: TextOverflow) -> List {
  List {
    items: list.items,
    fill_width: list.fill_width,
    window: list.window,
    inner_window: list.inner_window,
    width: list.width,
    height: list.height,
    selected_index: list.selected_index,
    scroll_top: list.scroll_top,
    text_overflow: overflow,
  }
}

pub fn list_fill_width(list: List, is_fill: bool) -> List {
  List {
    items: list.items,
    fill_width: is_fill,
    window: list.window,
    inner_window: list.inner_window,
    width: list.width,
    height: list.height,
    selected_index: list.selected_index,
    scroll_top: list.scroll_top,
    text_overflow: list.text_overflow,
  }
}

pub fn move_next_list(list: List) -> List {
  let is_in_range = list.selected_index < list.items.len() as i32 - 1;
  let new_index = if is_in_range {
    list.selected_index + 1
  } else {
    list.selected_index
  };
  let offset = list.selected_index + list.scroll_top;
  let new_scroll_top = if offset > list.height - 4 && is_in_range {
    list.scroll_top - 1
  } else {
    list.scroll_top
  };
  List {
    items: list.items,
    fill_width: list.fill_width,
    window: list.window,
    inner_window: list.inner_window,
    width: list.width,
    height: list.height,
    selected_index: new_index,
    scroll_top: new_scroll_top,
    text_overflow: list.text_overflow,
  }
}

pub fn move_prev_list(list: List) -> List {
  let is_in_range = list.selected_index > 0;
  let new_index = if is_in_range {
    list.selected_index - 1
  } else {
    list.selected_index
  };
  let offset = list.selected_index + list.scroll_top - 1;
  let new_scroll_top = if offset < 0 && is_in_range {
    list.scroll_top + 1
  } else {
    list.scroll_top
  };
  List {
    items: list.items,
    fill_width: list.fill_width,
    window: list.window,
    inner_window: list.inner_window,
    width: list.width,
    height: list.height,
    selected_index: new_index,
    scroll_top: new_scroll_top,
    text_overflow: list.text_overflow,
  }
}

pub fn render_list(list: &List) {
  let win = list.inner_window;
  wclear(win);
  box_(list.window, 0, 0);
  let inner_window_width = list.width - 4;
  let mut line = list.scroll_top;
  for item in &list.items {
    let item_width = item.chars().count() as i32;
    let is_overflow = item_width > inner_window_width;
    let overflow = match list.text_overflow {
      TextOverflow::Ellipsis => {
        if is_overflow {
          "..."
        } else {
          ""
        }
      }
      TextOverflow::Hidden => "",
    };
    let display_item = if list.fill_width {
      let new_item = if is_overflow {
        let take_length = (inner_window_width - 3) as usize;
        item.clone().chars().take(take_length).collect::<String>()
      } else {
        item.clone()
      };
      let spaces = if is_overflow {
        0
      } else {
        inner_window_width - item_width
      };
      format!("{}{}{}", new_item, " ".repeat(spaces as usize), overflow)
    } else {
      let new_item = if is_overflow {
        let take_length = (inner_window_width - 3) as usize;
        item.clone().chars().take(take_length).collect::<String>()
      } else {
        item.clone()
      };
      format!("{}{}", new_item, overflow)
    };
    if list.selected_index + list.scroll_top == line {
      wattr_on(win, A_REVERSE());
      mvwaddstr(win, line, 1, &display_item);
      wattr_off(win, A_REVERSE());
    } else {
      mvwaddstr(win, line, 1, &display_item);
    }
    line += 1;
  }
  wrefresh(list.window);
  wrefresh(win);
}