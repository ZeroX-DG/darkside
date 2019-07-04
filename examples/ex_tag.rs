extern crate darkside;

use darkside::tag::*;
use darkside::*;

fn main() {
  new_app();
  let tag = new_tag("Hello", 10, 10);
  render_tag(&tag);
  wait_for_key();
}