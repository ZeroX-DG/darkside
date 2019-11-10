extern crate darkside;
extern crate ncurses;

use darkside::{
    widgets::{List, Location, list::{ListItem, ListStyle}},
    App, ControlFlow,
};

fn main() {
    let mut app = App::new();

    // create list
    let listA = List::new(Location { x: 10, y: 0 })
        .items(vec![
            ListItem::new(String::from("Item A")).list_style(ListStyle::Disc).build(),
            ListItem::new(String::from("Item B")).list_style(ListStyle::Circle).build(),
            ListItem::new(String::from("Item C")).list_style(ListStyle::Square).build(),
        ])
        .build();

    let listB = List::new(Location { x: 10, y: 4 })
        .items(vec![
            ListItem::new(String::from("Item A")).list_style(ListStyle::UpperLatin).build(),
            ListItem::new(String::from("Item B")).list_style(ListStyle::UpperLatin).build(),
            ListItem::new(String::from("Item C")).list_style(ListStyle::UpperLatin).build(),
        ])
        .build();

    let listC = List::new(Location { x: 10, y: 8 })
        .items(vec![
            ListItem::new(String::from("Item A")).list_style(ListStyle::UpperRoman).build(),
            ListItem::new(String::from("Item B")).list_style(ListStyle::UpperRoman).build(),
            ListItem::new(String::from("Item C")).list_style(ListStyle::UpperRoman).build(),
        ])
        .build();

    // add text to app window
    app.window.children.push(listA);
    app.window.children.push(listB);
    app.window.children.push(listC);

    // render app
    app.run(Box::new(|| {
        ncurses::getch();
        ControlFlow::Break
    }));
}
