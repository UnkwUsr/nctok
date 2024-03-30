use crossterm::event::{self, Event, KeyCode};

use ratatui::{backend::Backend, widgets::TableState, Terminal};
use std::{io, ops::Add};

use crate::{entry::Entry, ui::ui, ConfigArgs};

pub struct App<'a> {
    pub config: ConfigArgs,
    pub cursor: TableState,
    // usize in history is tablestate before traversing down, so we can revert it back when going
    // up
    // str is entry name
    pub history: Vec<(&'a Entry, &'a str, usize)>,
}

impl<'a> App<'a> {
    pub fn new(root: &'a Entry, config: ConfigArgs) -> App<'a> {
        let mut cursor = TableState::default();
        cursor.select(Some(0));

        App {
            config,
            cursor,
            history: vec![(root, "", 0)],
        }
    }

    pub fn current(&self) -> &'a Entry {
        self.history.last().unwrap().0
    }
    pub fn under_cursor(&self) -> (&'a Entry, &'a str) {
        self.current()
            .children
            .as_ref()
            .unwrap()
            .iter()
            .nth(self.cursor.selected().unwrap())
            .map(|x| (x.1, x.0.as_ref()))
            .unwrap()
    }
    pub fn current_path(&self) -> String {
        self.history
            .iter()
            // first item is always empty string as we initializing it (in constructor)
            .skip(1)
            .map(|x| x.1)
            .fold("".to_string(), |x, y| {
                let mut t = x + y;
                t.push(self.config.parser.path_separator);
                t
            })
    }
    pub fn under_cursor_path(&self) -> String {
        self.current_path().add(self.under_cursor().1)
    }

    fn next(&mut self) {
        if let Some(childs) = &self.current().children {
            if let Some(i) = self.cursor.selected() {
                if i < (childs.len() - 1) {
                    self.cursor.select(Some(i + 1));
                }
            }
        }
    }

    fn previous(&mut self) {
        if let Some(i) = self.cursor.selected() {
            if i > 0 {
                self.cursor.select(Some(i - 1));
            }
        }
    }

    fn first(&mut self) {
        self.cursor.select(Some(0));
    }

    fn last(&mut self) {
        if let Some(childs) = &self.current().children {
            if !childs.is_empty() {
                self.cursor.select(Some(childs.len() - 1));
            }
        }
    }

    fn traverse_down(&mut self) {
        let new = self.under_cursor();
        // do not traverse into files
        if new.0.children.is_none() {
            return;
        }

        self.history
            .push((new.0, new.1, self.cursor.selected().unwrap()));
        self.cursor.select(Some(0));
    }

    fn traverse_up(&mut self) {
        if self.history.len() == 1 {
            return;
        }
        let (_, _, prev_state) = self.history.pop().unwrap();
        self.cursor.select(Some(prev_state));
    }

    fn toggle_preview(&mut self) {
        self.config.ui.preview ^= true;
    }
    fn toggle_human_readable(&mut self) {
        self.config.ui.human_readable ^= true;
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

                KeyCode::Char('w') => app.toggle_preview(),
                KeyCode::Char('a') => app.toggle_human_readable(),
                _ => {}
            }
        }
    }
}
