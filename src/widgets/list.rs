use super::{util, Renderable, BaseWidget, Location, Size};
use ncurses::{WINDOW, mvwaddstr};

pub struct List {
    base: BaseWidget,
    items: Vec<ListItem>
}

pub struct ListItem {
    list_style: Option<ListStyle>,
    content: String
}

pub enum ListStyle {
    Circle,
    Disc,
    Square,
    UpperRoman,
    UpperLatin,
    LowerRoman,
    LowerLatin,
    Number
}

impl List {
    pub fn new(location: Location) -> Self {
        List {
            base: BaseWidget::new(location, Size { width: 0, height: 0 }),
            items: Vec::new()
        }
    }

    pub fn items(mut self, items: Vec<ListItem>) -> Self {
        if self.base.size.width == 0 || self.base.size.height == 0 {
            let height = items.len() as u16;
            let width = items.iter()
                .max_by_key(|i| i.content.len())
                .unwrap()
                .content.len() as u16;
            let size = Size {
                width,
                height
            };
            self.base.size = size;
        }
        self.items = items;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.base.size = size;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

impl ListItem {
    pub fn new(content: String) -> Self {
        ListItem {
            content,
            list_style: None
        }
    }

    pub fn list_style(mut self, list_style: ListStyle) -> Self {
        self.list_style = Some(list_style);
        self
    }

    pub fn no_list_style(mut self) -> Self {
        self.list_style = None;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

fn get_list_item_prefix(index: u64, list_style: &Option<ListStyle>) -> String {
    if let Some(style) = list_style {
        return match style {
            ListStyle::Number     => format!("{}. ", index),
            ListStyle::Circle     => format!("○ "),
            ListStyle::Disc       => format!("● "),
            ListStyle::Square     => format!("■ "),
            ListStyle::UpperRoman => format!("{}. ", util::num_to_roman(index + 1)),
            ListStyle::LowerRoman => format!("{}. ", util::num_to_roman(index + 1).to_lowercase()),
            ListStyle::UpperLatin => format!("{}. ", util::num_to_latin(index)),
            ListStyle::LowerLatin => format!("{}. ", util::num_to_latin(index).to_lowercase())
        }
    }
    String::new()
}

impl Renderable for List {
    fn render(&self, window: WINDOW) {
        let mut index = 0;
        let renderable_items = self.items
            .iter()
            .map(|i| {
                let mut out = get_list_item_prefix(index, &i.list_style);
                out.push_str(&i.content.clone());
                index += 1;
                out
            })
            .collect::<Vec<String>>();

        let mut y = self.base.position.y as i32;
        let x = self.base.position.x as i32;
        for item in renderable_items.iter() {
            mvwaddstr(window, y, x, item);
            y += 1;
        }
    }
}
