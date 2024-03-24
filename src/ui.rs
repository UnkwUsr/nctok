use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

use crate::{app::App, entry::Entry};

pub fn ui(f: &mut Frame, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let t_main = make_table(app.current());
    let t_preview = make_table(app.under_cursor());

    f.render_stateful_widget(t_main, chunks[0], &mut app.state);
    f.render_stateful_widget(t_preview, chunks[1], &mut TableState::default());
}

fn make_table(entry: &Entry) -> Table<'static> {
    let rows = entry
        .children
        .as_ref()
        .map(|y| {
            y.iter()
                .map(|x| {
                    let (number, suffix) = styled_number(x.1.size);
                    let name = styled_name(x.0.to_owned(), x.1.children.is_some());

                    let cells = [number, suffix, name];
                    Row::new(cells).height(1)
                })
                .collect()
        })
        .unwrap_or(Vec::new());

    let widths = &[
        Constraint::Max(5),
        Constraint::Max(2), // actually we need only 1, but one more just for padding
        Constraint::Fill(1),
    ];
    Table::new(rows, widths)
        // .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(Style::default().bg(ratatui::style::Color::Green))
        .highlight_symbol(">> ")
}

/// format human readable (for big numbers) and colorize
fn styled_number(size: usize) -> (Cell<'static>, Cell<'static>) {
    let t = if size < 1000 {
        (Cell::from(size.to_string()), Cell::default())
    } else {
        human_format::Formatter::default()
            .with_decimals(1)
            .format(size as f64)
            .split_once(" ")
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
