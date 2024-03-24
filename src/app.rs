use crossterm::event::{self, Event, KeyCode};

use std::io;
use tui::{backend::Backend, widgets::TableState, Terminal};

use crate::{entry::Entry, ui::ui};

pub struct App<'a> {
    pub state: TableState,
    pub history: Vec<&'a Entry>,
}

impl<'a> App<'a> {
    pub fn new(root: &'a Entry) -> App<'a> {
        let mut state = TableState::default();
        state.select(Some(0));

        App {
            state,
            history: vec![root],
        }
    }

    pub fn current(&self) -> &Entry {
        self.history.last().unwrap()
    }

    fn next(&mut self) {
        if let Some(childs) = &self.current().children {
            if let Some(i) = self.state.selected() {
                if i < (childs.len() - 1) {
                    self.state.select(Some(i + 1));
                }
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
        if let Some(childs) = &self.current().children {
            if !childs.is_empty() {
                self.state.select(Some(childs.len() - 1));
            }
        }
    }

    fn traverse_down(&mut self) {
        // TODO: here is hack to fight borrow checker, can't just call self.current()
        let asd = self.history.last().unwrap();

        let cur = asd
            .children
            .as_ref()
            .unwrap()
            .iter()
            .rev()
            .nth(self.state.selected().unwrap())
            .unwrap();
        self.history.push(cur.1);
        self.state.select(Some(0));
    }

    fn traverse_up(&mut self) {
        if self.history.len() == 1 {
            return;
        }
        self.history.pop();
        self.state.select(Some(0));
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
                KeyCode::Char('l') => app.traverse_down(),
                KeyCode::Char('h') => app.traverse_up(),
                KeyCode::Char('g') => app.first(),
                KeyCode::Char('G') => app.last(),
                _ => {}
            }
        }
    }
}
