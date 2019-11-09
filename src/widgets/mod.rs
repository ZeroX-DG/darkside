use ncurses::WINDOW;

pub mod text;
mod util;
pub mod window;

pub use text::Text;
pub use window::Window;

pub struct BaseWidget {
    position: Location,
    size: Size,
}

pub struct Location {
    pub x: u16,
    pub y: u16,
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub trait Renderable {
    fn render(&self, window: WINDOW);
}

impl BaseWidget {
    pub fn new(position: Location, size: Size) -> Self {
        BaseWidget { position, size }
    }
}
