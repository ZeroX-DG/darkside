use ncurses::*;

pub struct List {
  items: Vec<String>,
  fill_width: bool,
  window: WINDOW,
  width: i32,
  height: i32,
  selected_index: i32,
  scroll_top: i32,
  text_overflow: TextOverflow,
  item_height: i32,
  item_spacing: i32,
  visible: bool,
}

pub enum TextOverflow {
  Ellipsis,
  Hidden,
}

/// Create a new list widget
pub fn new_list(x: i32, y: i32, w: i32, h: i32, items: Vec<String>) -> List {
  let win = newwin(h, w, y, x);
  List {
    items: items,
    fill_width: false,
    window: win,
    width: w,
    height: h,
    selected_index: 0,
    scroll_top: 0,
    text_overflow: TextOverflow::Ellipsis,
    item_height: 1,
    item_spacing: 0,
    visible: true,
  }
}

/// Set text overflow effect for the list
pub fn set_list_text_overflow(list: List, overflow: TextOverflow) -> List {
  let mut update_list = list;
  update_list.text_overflow = overflow;
  update_list
}

/// Allow the list item to fill the list width
pub fn set_list_fill_width(list: List, is_fill: bool) -> List {
  let mut update_list = list;
  update_list.fill_width = is_fill;
  update_list
}

/// Select the next item in list
pub fn move_next_list_item(list: List) -> List {
  let is_in_range = list.selected_index < list.items.len() as i32 - 1;
  let new_index = if is_in_range {
    list.selected_index + 1
  } else {
    list.selected_index
  };
  let offset_extra = list.item_height + list.item_spacing;
  let offset = list.selected_index * offset_extra + list.scroll_top;
  let new_scroll_top = if offset > list.height - (1 + offset_extra) && is_in_range {
    list.scroll_top - offset_extra
  } else {
    list.scroll_top
  };
  let mut update_list = list;
  update_list.selected_index = new_index;
  update_list.scroll_top = new_scroll_top;
  update_list
}

/// Select the previous item in list
pub fn move_prev_list_item(list: List) -> List {
  let is_in_range = list.selected_index > 0;
  let new_index = if is_in_range {
    list.selected_index - 1
  } else {
    list.selected_index
  };
  let offset_extra = list.item_height + list.item_spacing;
  let offset = list.selected_index * offset_extra + list.scroll_top - 1;
  let new_scroll_top = if offset < 0 && is_in_range {
    list.scroll_top + offset_extra
  } else {
    list.scroll_top
  };
  let mut update_list = list;
  update_list.selected_index = new_index;
  update_list.scroll_top = new_scroll_top;
  update_list
}

/// Set the list item height
pub fn set_list_item_height(list: List, height: i32) -> List {
  let mut update_list = list;
  update_list.item_height = height;
  update_list
}

/// Set the list item spacing
pub fn set_list_item_spacing(list: List, spacing: i32) -> List {
  let mut update_list = list;
  update_list.item_spacing = spacing;
  update_list
}

/// Set the list visibility
pub fn set_list_visible(list: List, visible: bool) -> List {
  let mut update_list = list;
  update_list.visible = visible;
  update_list
}

/// Set the list items
pub fn set_list_items(list: List, items: Vec<String>) -> List {
  let mut update_list = list;
  update_list.items = items;
  update_list
}

/// Get current selected item
pub fn get_list_selected_index(list: &List) -> i32 {
  list.selected_index
}

/// Render the list
pub fn render_list(list: &List) {
  let win = list.window;
  wclear(win);
  if !list.visible {
    wrefresh(win);
    return;
  }
  let mut line = list.scroll_top;
  for item in &list.items {
    let mut display_lines: Vec<String> = vec![];
    for item_line in item.lines() {
      let item_str = String::from(item_line);
      let item_width = item_str.chars().count() as i32;
      let is_overflow = item_width > list.width;
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
          let take_length = (list.width - overflow.len() as i32) as usize;
          item_str
            .clone()
            .chars()
            .take(take_length)
            .collect::<String>()
        } else {
          item_str.clone()
        };
        let spaces = if is_overflow {
          0
        } else {
          list.width - item_width
        };
        format!("{}{}{}", new_item, " ".repeat(spaces as usize), overflow)
      } else {
        let new_item = if is_overflow {
          let take_length = (list.width - overflow.len() as i32) as usize;
          item_str
            .clone()
            .chars()
            .take(take_length)
            .collect::<String>()
        } else {
          item_str.clone()
        };
        format!("{}{}", new_item, overflow)
      };
      display_lines.push(display_item);
    }
    let offset = list.item_height + list.item_spacing;
    let mut sub_line = line;
    for display_line in display_lines {
      if list.selected_index * offset + list.scroll_top == line {
        wattr_on(win, A_REVERSE());
        mvwaddstr(win, sub_line, 0, &display_line);
        wattr_off(win, A_REVERSE());
      } else {
        mvwaddstr(win, sub_line, 0, &display_line);
      }
      sub_line += 1;
    }
    line += offset;
  }
  wrefresh(list.window);
  wrefresh(win);
}
