use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{backend::Backend, widgets::TableState, Terminal};

use crate::ui::ui;

#[derive(Debug)]
pub struct MyListItem {
    pub direntry: String,
    pub count: usize,
}

pub struct App {
    pub state: TableState,
    pub items: Vec<MyListItem>,
}

impl App {
    pub fn new() -> App {
        let mut state = TableState::default();
        state.select(Some(0));

        App {
            state,
            items: get_list_items(),
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn get_list_items() -> Vec<MyListItem> {
    let config = tokei::Config::default();

    let mut items = std::fs::read_dir(".")
        .unwrap()
        .map(|x| x.unwrap().path())
        .map(|x| {
            let mut languages = tokei::Languages::new();
            languages.get_statistics(&[&x], &[], &config);

            let mut direntry = x.display().to_string();
            direntry.drain(0..2);

            MyListItem {
                direntry,
                count: languages.total().code,
            }
        })
        .collect::<Vec<MyListItem>>();
    // TODO: maybe sort in order: folders first
    items.sort_by_key(|x| x.count);
    items.reverse();

    items
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') => app.next(),
                KeyCode::Char('k') => app.previous(),
                _ => {}
            }
        }
    }
}