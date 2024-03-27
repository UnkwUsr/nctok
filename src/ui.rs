use clap::ArgAction;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

use crate::{app::App, entry::Entry};

#[derive(clap::Args)]
pub struct UiConfig {
    #[rustfmt::skip]
    #[arg(long, default_value = "false", help = "Show preview window for entry under cursor (can also toggle with 'w' key)")]
    pub preview: bool,
    #[rustfmt::skip]
    #[arg(long = "no-human-readable", default_value = "true", action = ArgAction::SetFalse, help = "Disable formatting big numbers in human-readable (can also toggle with 'a' key)")]
    pub human_readable: bool,
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let constraints = if app.config.ui.preview {
        vec![Constraint::Percentage(50); 2]
    } else {
        vec![Constraint::Percentage(100)]
    };
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(f.size());

    let t_main = make_table(app.current(), &app.config.ui);
    f.render_stateful_widget(t_main, chunks[0], &mut app.state);

    if app.config.ui.preview {
        let t_preview = make_table(app.under_cursor(), &app.config.ui);
        f.render_stateful_widget(t_preview, chunks[1], &mut TableState::default());
    }
}

fn make_table(entry: &Entry, config: &UiConfig) -> Table<'static> {
    let rows: Vec<Row> = entry
        .children
        .as_ref()
        .map(|y| {
            y.iter()
                .map(|x| {
                    let (number, suffix) = styled_number(x.1.size, config.human_readable);
                    let name = styled_name(x.0.to_owned(), x.1.children.is_some());

                    let cells = [number, suffix, name];
                    Row::new(cells).height(1)
                })
                .collect()
        })
        .unwrap_or_default();

    let widths = make_widths(config, get_max_size(entry));
    Table::new(rows, widths)
        // .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(Style::default().bg(ratatui::style::Color::Green))
        .highlight_symbol(">> ")
}

/// format human readable (for big numbers) and colorize
fn styled_number(size: usize, human_readable: bool) -> (Cell<'static>, Cell<'static>) {
    let t = if size < 1000 || !human_readable {
        (Cell::from(size.to_string()), Cell::default())
    } else {
        human_format::Formatter::default()
            .with_decimals(1)
            .format(size as f64)
            .split_once(' ')
            .map(|(x, y)| (Cell::from(x.to_owned()), Cell::from(y.to_owned())))
            .unwrap()
    };

    (
        t.0.style(
            Style::default()
                .fg(ratatui::style::Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        t.1,
    )
}

/// colorizing if entry have children
fn styled_name(name: String, have_children: bool) -> Cell<'static> {
    let t = Cell::from(name);
    if have_children {
        t.style(
            Style::default()
                .fg(ratatui::style::Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        t
    }
}

fn make_widths(config: &UiConfig, max: usize) -> [Constraint; 3] {
    let max_len = (max.ilog10() + 1) as u16;
    let number = if config.human_readable { 5 } else { max_len };
    // for suffix 1 character is just enough, but we add one more for visual padding
    let suffix = if config.human_readable { 2 } else { 0 };

    [
        Constraint::Max(number),
        Constraint::Max(suffix),
        Constraint::Fill(1),
    ]
}

fn get_max_size(entry: &Entry) -> usize {
    entry
        .children
        .as_ref()
        .iter()
        .next()
        .map_or(1, |x| x.first().unwrap().1.size)
}
