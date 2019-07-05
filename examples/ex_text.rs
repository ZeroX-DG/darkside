extern crate darkside;

use darkside::text::*;
use darkside::region::*;
use darkside::*;

fn main() {
  new_app();
  let mut hello_text = new_text("Hello, world", 0, 0);
  hello_text = set_text_effects(hello_text, vec![TextEffect::Italic, TextEffect::Bold]);
  hello_text = center_text(hello_text, true, true);

  let region = new_region(0, 0, 20, 5, None, Border::All);
  let mut top_left_text = new_text("Top", 3, 2);
  top_left_text = set_text_region(top_left_text, &region);

  // Region must be rendered before the text that it contains
  render_region(&region);
  render_text(&top_left_text);
  render_text(&hello_text);
  wait_for_key();
}
