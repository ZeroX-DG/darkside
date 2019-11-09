use super::{BaseWidget, Location, Renderable, Size};
use ncurses::{attr_off, attr_on, attr_t, mvwaddstr};

pub struct Text {
    base: BaseWidget,
    content: String,
    effects: Vec<TextEffect>,
}

pub enum TextEffect {
    Bold,
    Underline,
    Italic,
}

impl Text {
    pub fn new(location: Location) -> Self {
        let render_size = Size {
            height: 0,
            width: 0,
        };
        Text {
            base: BaseWidget::new(location, render_size),
            content: String::new(),
            effects: Vec::new(),
        }
    }

    pub fn effects(mut self, effects: Vec<TextEffect>) -> Self {
        self.effects = effects;
        self
    }

    pub fn content(mut self, text: String) -> Self {
        let height = text.lines().count() as u16;
        let width = text.lines().max_by_key(|l| l.len()).unwrap().len() as u16;
        let size = Size { width, height };
        self.base.size = size;
        self.content = text;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

fn text_effect_to_attribute(effect: &TextEffect) -> attr_t {
    match effect {
        TextEffect::Bold => ncurses::A_BOLD(),
        TextEffect::Italic => ncurses::A_ITALIC(),
        TextEffect::Underline => ncurses::A_UNDERLINE(),
    }
}

impl Renderable for Text {
    fn render(&self, window: ncurses::WINDOW) {
        let position = &self.base.position;
        let size = &self.base.size;
        let renderable_text = self
            .content
            .lines()
            .take(size.height as usize)
            .map(|l| l[0..size.width as usize].to_string())
            .collect::<String>();
        for effect in self.effects.iter() {
            let attr = text_effect_to_attribute(effect);
            attr_on(attr);
        }
        mvwaddstr(
            window,
            position.x as i32,
            position.y as i32,
            &renderable_text,
        );
        for effect in self.effects.iter() {
            let attr = text_effect_to_attribute(effect);
            attr_off(attr);
        }
    }
}
