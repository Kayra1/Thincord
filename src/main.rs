use std::{io};
use tui::{backend::CrosstermBackend, Terminal };
use crossterm::{ execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen} };

mod app;
use crate::app::{main_loop};

fn main() -> Result<(), io::Error> {
    // 1. Enable Terminal
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // 2. Run Program
    
    let result = main_loop(&mut terminal);

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
