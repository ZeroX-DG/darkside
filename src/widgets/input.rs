use ncurses::*;

pub struct Input<'a> {
  prompt: &'a str,
  obscure: bool,
  value: String,
  window: WINDOW,
  width: i32,
  visible: bool,
}


/// Create a new input
pub fn new_input<'a>(x: i32, y: i32, w: i32, prompt: &'a str, obscure: bool) -> Input<'a> {
  let win = newwin(1, w, y, x);
  Input {
    prompt: prompt,
    obscure: obscure,
    value: String::from(""),
    window: win,
    width: w,
    visible: true,
  }
}

/// Set the input value
pub fn set_input_value(input: Input, value: String) -> Input {
  let mut update_input = input;
  update_input.value = value;
  update_input
}

/// Specify if the input value should be obscure or not
pub fn set_input_obscure(input: Input, obscure: bool) -> Input {
  let mut update_input = input;
  update_input.obscure = obscure;
  update_input
}

/// Specify if the input value should be displayed or not
pub fn set_input_visible(input: Input, visible: bool) -> Input {
  let mut update_input = input;
  update_input.visible = visible;
  update_input
}

/// Set new prompt for input
pub fn set_input_prompt<'a>(input: Input<'a>, prompt: &'a str) -> Input<'a> {
  let mut update_input = input;
  update_input.prompt = prompt;
  update_input
}

/// Return the current input value
pub fn get_input_value(input: &Input) -> String {
  input.value.clone()
}

/// Add a new char to the end of the input value
pub fn add_input_char(input: Input, input_char: char) -> Input {
  let mut update_input = input;
  update_input.value.push(input_char);
  update_input
}

/// Remove the last char from the input value
pub fn remove_input_last_char(input: Input) -> Input {
  let mut update_input = input;
  let char_count: i32 = update_input.value.chars().count() as i32;
  let index: i32 = if char_count > 0 { char_count - 1 } else { -1 };
  if index >= 0 {
    update_input.value.remove(index as usize);
  }
  update_input
}

/// Render the input widget
pub fn render_input(input: &Input) {
  let win = input.window;
  wclear(win);

  if !input.visible {
    wrefresh(input.window);
    return;
  }

  let prompt_width = input.prompt.chars().count();

  wattr_on(win, A_BOLD());
  mvwaddstr(win, 0, 0, input.prompt);
  wattr_off(win, A_BOLD());

  mvwaddstr(win, 0, prompt_width as i32, ": ");
  let value_width = input.width - prompt_width as i32 - 2;
  let value_length = input.value.chars().count();
  let value = if input.obscure {
    "*".repeat(value_length)
  } else {
    input
      .value
      .clone()
      .chars()
      .take(value_width as usize)
      .collect::<String>()
  };
  mvwaddstr(win, 0, prompt_width as i32 + 2, &value);
  wrefresh(input.window);
}
