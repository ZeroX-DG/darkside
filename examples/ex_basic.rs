extern crate darkside;

use darkside::*;

fn main() {
  new_app();
  let mut hello_text = text::new_text(String::from("Hello, world"), 0, 0);
  hello_text = text::set_text_effects(
    hello_text,
    vec![text::TextEffect::ITALIC, text::TextEffect::BOLD],
  );
  hello_text = text::center_text(hello_text, true, true);
  text::render(hello_text);
  pause_app();
}