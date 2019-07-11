pub mod input;
pub mod list;
pub mod region;
pub mod tag;
pub mod text;

#[derive(Clone)]
pub enum Border {
  Left,
  Top,
  Right,
  Bottom,
  All,
  None,
}
