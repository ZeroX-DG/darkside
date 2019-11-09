pub mod widgets;

use ncurses::{initscr, refresh, endwin};
use widgets::Renderable;

pub struct App<R: Renderable> {
    pub window: widgets::Window<R>
}

pub enum ControlFlow {
    Continue,
    Break
}

impl<R: Renderable> App<R> {
    pub fn new() -> App<R> {
        let window = initscr();
        refresh();
        App {
            window: widgets::Window {
                children: Vec::new(),
                border: widgets::window::WindowBorder::NoBorder,
                window
            }
        }
    }

    pub fn run<F>(&self, mut callback: Box<F>) where
        F: FnMut() -> ControlFlow {
        self.window.render(self.window.window);
        loop {
            match callback() {
                ControlFlow::Break => break,
                ControlFlow::Continue => {
                    self.window.render(self.window.window);
                }
            }
        }
        endwin();
    }
}
