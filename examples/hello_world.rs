extern crate darkside;
extern crate ncurses;

use darkside::{
    widgets::{text::TextEffect, Location, Text},
    App, ControlFlow,
};

fn main() {
    let mut app = App::new();

    // create text
    let text = Text::new(Location { x: 10, y: 20 })
        .content(String::from("Hello world"))
        .effects(vec![
            TextEffect::Bold,
            TextEffect::Italic,
            TextEffect::Underline,
            TextEffect::Dim
        ])
        .build();

    // add text to app window
    app.window.children.push(text);

    // render app
    app.run(Box::new(|| {
        ncurses::getch();
        ControlFlow::Break
    }));
}
