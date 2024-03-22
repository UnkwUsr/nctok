use crossterm::event::{self, Event, KeyCode};

use std::io;
use tui::{backend::Backend, widgets::TableState, Terminal};

use crate::{entry::Entry, ui::ui};

pub struct App {
    pub state: TableState,
    pub root: Entry,
    pub cur: Entry,
}

impl App {
    pub fn new(root: Entry) -> App {
        let mut state = TableState::default();
        state.select(Some(0));

        App {
            state,
            cur: root,
            // TODO: very stub. root could be root, and cur somehow reference
            root: Entry {
                size: 0,
                children: None,
            },
        }
    }

    fn next(&mut self) {
        if let Some(i) = self.state.selected() {
            if i < (self.cur.children.as_ref().unwrap().len() - 1) {
                self.state.select(Some(i + 1));
            }
        }
    }

    fn previous(&mut self) {
        if let Some(i) = self.state.selected() {
            if i > 0 {
                self.state.select(Some(i - 1));
            }
        }
    }

    fn first(&mut self) {
        self.state.select(Some(0));
    }

    fn last(&mut self) {
        if !self.cur.children.as_ref().unwrap().is_empty() {
            self.state
                .select(Some(self.cur.children.as_ref().unwrap().len() - 1));
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') => app.next(),
                KeyCode::Char('k') => app.previous(),
                KeyCode::Char('g') => app.first(),
                KeyCode::Char('G') => app.last(),
                _ => {}
            }
        }
    }
}
