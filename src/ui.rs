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
                    let cells = [
                        Cell::from(x.1.size.to_string()),
                        // colorizing if entry have children
                        {
                            let t = Cell::from(x.0.to_string());
                            if x.1.children.is_some() {
                                t.style(
                                    Style::default()
                                        .fg(ratatui::style::Color::LightBlue)
                                        .add_modifier(Modifier::BOLD),
                                )
                            } else {
                                t
                            }
                        },
                    ];
                    Row::new(cells).height(1)
                })
                .collect()
        })
        .unwrap_or(Vec::new());

    let widths = &[
        Constraint::Percentage(50),
        Constraint::Length(30),
        Constraint::Min(10),
    ];
    Table::new(rows, widths)
        // .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(Style::default().bg(ratatui::style::Color::Green))
        .highlight_symbol(">> ")
}
