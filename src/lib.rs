mod edit_field;
mod row;
mod item_list;
mod proxy;
pub mod style;
mod text;
mod column;
mod group;
mod dummy;

use ncurses::CURSOR_VISIBILITY::*;
use ncurses::*;
use std::panic::{set_hook, take_hook};
use std::collections::VecDeque;

pub use self::edit_field::*;
pub use self::row::*;
pub use self::item_list::*;
pub use self::proxy::*;
pub use self::text::*;
pub use self::column::*;
pub use self::group::*;
pub use self::dummy::*;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub enum Event {
    Quit,
    KeyStroke(i32),
    Message(String),
}


pub trait Widget {
    fn render(&mut self, context: &mut Context, rect: &Rect, active: bool);
    fn handle_event(&mut self, context: &mut Context, event: &Event);
}

pub fn screen_rect() -> Rect {
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    getmaxyx(stdscr(), &mut h, &mut w);
    Rect {
        x: 0.0,
        y: 0.0,
        w: w as f32,
        h: h as f32,
    }
}

pub struct Context {
    pub event_queue: VecDeque<Event>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            event_queue: VecDeque::new()
        }
    }

    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    pub fn exec(&mut self, mut ui: Box<dyn Widget>) {
        initscr();

        start_color();
        init_pair(style::REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
        init_pair(style::CURSOR_PAIR, COLOR_BLACK, COLOR_WHITE);
        init_pair(style::INACTIVE_CURSOR_PAIR, COLOR_BLACK, COLOR_CYAN);

        curs_set(CURSOR_INVISIBLE);

        set_hook(Box::new({
            let default_hook = take_hook();
            move |payload| {
                endwin();
                default_hook(payload);
            }
        }));

        let mut quit = false;
        while !quit {
            erase();
            ui.render(self, &screen_rect(), true);
            let key = getch();
            self.event_queue.push_back(Event::KeyStroke(key));
            while !self.event_queue.is_empty() {
                self.event_queue.pop_front().map(|event| match event {
                    // TODO: maybe we should propagate the Quit event down the ui tree as well?
                    Event::Quit => quit = true,
                    _ => ui.handle_event(self, &event),
                });
            }
        }

        endwin();
    }

    pub fn quit(&mut self) {
        self.push_event(Event::Quit);
    }
}
