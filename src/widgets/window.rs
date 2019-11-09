use super::{Renderable, Location, Size};
use ncurses::{WINDOW, wborder};
use super::util::*;

pub enum WindowBorder {
    NoBorder,
    Dashed,
    Solid,
    Custom((char, char, char, char, char, char))
}

impl WindowBorder {
    fn has_border(&self) -> bool {
        match *self {
            WindowBorder::NoBorder => true,
            _ => false
        }
    }
}

pub struct Window<R: Renderable> {
    pub children: Vec<R>,
    pub border: WindowBorder,
    pub window: WINDOW,
}

impl<R: Renderable> Window<R> {
    pub fn new(location: Location, size: Size, border: WindowBorder) -> Self {
        let window = ncurses::newwin(
            size.height as i32,
            size.width as i32,
            location.y as i32,
            location.x as i32
        );

        Window {
            children: Vec::new(),
            border,
            window,
        }
    }
}

impl<R: Renderable> Renderable for Window<R> {
    fn render(&self, _window: WINDOW) {
        if self.border.has_border() {
            let border_chars: Option<
                (char, char, char, char, char, char)
            > = match self.border {
                WindowBorder::Solid => Some((
                    SOLID_HLINE_CHAR,
                    SOLID_VLINE_CHAR,
                    SOLID_TLCORNER_CHAR,
                    SOLID_TRCORNER_CHAR,
                    SOLID_BLCORNER_CHAR,
                    SOLID_BRCORNER_CHAR
                )),
                WindowBorder::Dashed => Some((
                    DASHED_HLINE_CHAR,
                    DASHED_VLINE_CHAR,
                    DASHED_TLCORNER_CHAR,
                    DASHED_TRCORNER_CHAR,
                    DASHED_BLCORNER_CHAR,
                    DASHED_BRCORNER_CHAR
                )),
                WindowBorder::Custom(chars) => Some(chars),
                WindowBorder::NoBorder => None
            };

            if let Some(chars) = border_chars {
                wborder(
                    self.window,
                    chars.1 as u32,
                    chars.1 as u32,
                    chars.0 as u32,
                    chars.0 as u32,
                    chars.2 as u32,
                    chars.3 as u32,
                    chars.4 as u32,
                    chars.5 as u32
                );
            }
        }

        for child in self.children.iter() {
            child.render(self.window);
        }
    }
}
