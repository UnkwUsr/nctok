use clap::ArgAction;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
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

    let t_main = make_table(app.current(), app.current_path(), &app.config.ui);
    f.render_stateful_widget(t_main, chunks[0], &mut app.cursor);

    if app.config.ui.preview {
        let entry = app.under_cursor().0;
        let path = app.under_cursor_path();
        let t_preview = make_table(entry, path, &app.config.ui);
        f.render_stateful_widget(t_preview, chunks[1], &mut TableState::default());
    }
}

fn make_table(entry: &Entry, path: String, config: &UiConfig) -> Table<'static> {
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title_top(path.green().bold())
                .title_bottom(format!(
                    "Total size: {}",
                    number_humanized(entry.size, config.human_readable)
                )),
        )
        .highlight_style(Style::default().bg(ratatui::style::Color::Green))
        .highlight_symbol(">> ")
}

/// format human readable (for big numbers) and colorize
fn styled_number(num: usize, human_readable: bool) -> (Cell<'static>, Cell<'static>) {
    let (number, suffix) = number_humanized(num, human_readable)
        .split_once(' ')
        .map(|(x, y)| (Cell::from(x.to_owned()), Cell::from(y.to_owned())))
        .unwrap();

    (
        number.style(
            Style::default()
                .fg(ratatui::style::Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        suffix,
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

fn number_humanized(num: usize, human_readable: bool) -> String {
    if human_readable {
        human_format::Formatter::default()
            .with_decimals(if num < 1000 { 0 } else { 1 })
            .format(num as f64)
    } else {
        // adding space for convenience. Here is invisible "suffix" after it (we then splitting by
        // space)
        num.to_string() + " "
    }
}
