extern crate darkside;

use darkside::text::*;
use darkside::*;

fn main() {
  new_app();
  let mut hello_text = new_text(String::from("Hello, world"), 0, 0);
  hello_text = set_text_effects(hello_text, vec![TextEffect::ITALIC, TextEffect::BOLD]);
  hello_text = center_text(hello_text, true, true);
  render_text(hello_text);
  wait_for_key();
}