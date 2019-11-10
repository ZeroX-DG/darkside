use super::{BaseWidget, Renderable, Location, Size};
use ncurses::{WINDOW, wborder, mvwaddstr, getmaxyx, getyx};
use super::util::*;

pub enum WindowBorder {
    Dashed,
    Solid,
    Custom((char, char, char, char, char, char))
}

pub enum WindowTitleAlign {
    Left,
    Right,
    Center
}

pub enum WindowTitleStyle {
    Highlight,
    Inline
}

pub struct WindowTitle {
    pub align: WindowTitleAlign,
    pub content: String,
    pub style: WindowTitleStyle
}

pub struct Window<R: Renderable> {
    pub children: Vec<R>,
    pub window: WINDOW,
    pub border: Option<WindowBorder>,
    pub title: Option<WindowTitle>,
    pub base: BaseWidget
}

impl<R: Renderable> Window<R> {
    pub fn new(location: Location, size: Size) -> Self {
        let window = ncurses::newwin(
            size.height as i32,
            size.width as i32,
            location.y as i32,
            location.x as i32
        );

        Window {
            children: Vec::new(),
            border: None,
            window,
            title: None,
            base: BaseWidget::new(location, size)
        }
    }

    pub fn border(mut self, border: Option<WindowBorder>) -> Self {
        self.border = border;
        self
    }

    pub fn title(mut self, title: Option<WindowTitle>) -> Self {
        self.title = title;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

impl<R: Renderable> From<WINDOW> for Window<R> {
    fn from(window: WINDOW) -> Window<R> {
        let mut width = 0;
        let mut height = 0;
        let mut x = 0;
        let mut y = 0;

        getyx(window, &mut y, &mut x);
        getmaxyx(window, &mut height, &mut width);

        Window {
            children: Vec::new(),
            border: None,
            window,
            title: None,
            base: BaseWidget::new(Location {
                x: x as u16,
                y: y as u16
            }, Size {
                width: width as u16,
                height: height as u16
            })
        }
    }
}

impl<R: Renderable> Renderable for Window<R> {
    fn render(&self, _window: WINDOW) {
        if let Some(border) = &self.border {
            let border_chars:
                (char, char, char, char, char, char) = match border {
                WindowBorder::Solid => (
                    SOLID_HLINE_CHAR,
                    SOLID_VLINE_CHAR,
                    SOLID_TLCORNER_CHAR,
                    SOLID_TRCORNER_CHAR,
                    SOLID_BLCORNER_CHAR,
                    SOLID_BRCORNER_CHAR
                ),
                WindowBorder::Dashed => (
                    DASHED_HLINE_CHAR,
                    DASHED_VLINE_CHAR,
                    DASHED_TLCORNER_CHAR,
                    DASHED_TRCORNER_CHAR,
                    DASHED_BLCORNER_CHAR,
                    DASHED_BRCORNER_CHAR
                ),
                WindowBorder::Custom(chars) => *chars,
            };

            wborder(
                self.window,
                border_chars.1 as u32,
                border_chars.1 as u32,
                border_chars.0 as u32,
                border_chars.0 as u32,
                border_chars.2 as u32,
                border_chars.3 as u32,
                border_chars.4 as u32,
                border_chars.5 as u32
            );
        }

        if let Some(title) = &self.title {
            let title_len = title.content.chars().count() as u16;
            let renderable_title_width = if title_len > self.base.size.width {
                self.base.size.width
            } else {
                title_len as u16
            };
            let renderable_title_content = title.content
                .chars()
                .take(renderable_title_width as usize)
                .collect::<String>();
            let base_width = self.base.size.width;
            let x = match title.align {
                WindowTitleAlign::Left => 0,
                WindowTitleAlign::Right => base_width - renderable_title_width,
                WindowTitleAlign::Center => (base_width - renderable_title_width) / 2
            };
            mvwaddstr(self.window, 0, x as i32, &renderable_title_content);
        }

        for child in self.children.iter() {
            child.render(self.window);
        }
    }
}
