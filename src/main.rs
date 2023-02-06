use std::{io};
use tui::{backend::CrosstermBackend, Terminal };
use crossterm::{ execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen} };

mod thincord;
use crate::thincord::start;
use crate::thincord::State;

fn main() -> Result<(), io::Error> {
    // 1. Enable Terminal
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // 2. Run Program
    let mut state: State = State::new();
    let result = start(&mut terminal, &mut state);

    // 3. Disable Terminals
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        println!("{}", e.to_string())
    }
    Ok(())
}
