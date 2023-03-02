mod frontend;
mod backend;

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::{Backend}, Terminal};
use backend::{AppState, Operator};

pub fn main_loop<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), std::io::Error>{
    let mut app_state: AppState = AppState::new();
    loop {
        terminal.draw(|f| frontend::main_ui(f, &mut app_state));

        let event: Event = event::read()?;
        match event{
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Paste(_) => unimplemented!(),
            Event::Resize(_, _) => todo!(),
            Event::Key(e) => {
                if e.code == KeyCode::Char('q') {break}
                if e.code == KeyCode::Enter {app_state.set_last_operator(Operator::Enter)}
                if e.code == KeyCode::Backspace {app_state.input_text_mut().pop();}
                if let KeyCode::Char(c) = e.code {app_state.input_text_mut().push(c)}
            },
            Event::Mouse(_) => todo!(),
        }
    }
    Ok(())
}