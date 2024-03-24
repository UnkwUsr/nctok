mod app;
mod entry;
mod parser;
mod ui;

use app::{run_app, App};
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

#[derive(clap::Parser)]
#[command(version)]
#[command(about)]
struct ConfigArgs {
    #[arg(long, help = "Invert sort order", default_value = "false")]
    pub reverse: bool,

    #[command(flatten)]
    pub parser: parser::ParserConfig,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = ConfigArgs::parse();
    let root = parser::parse_stdin(config);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(&root);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
