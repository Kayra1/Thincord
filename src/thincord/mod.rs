use crossterm::event::{self, Event, KeyCode};
use tui::{backend::{Backend}, Terminal};

mod frontend;
// mod backend;

enum Menu {
    Login,
    Main,
    Settings
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    None,
    Enter,
    Escape
}

pub struct AppState {
    // Authentication State
    logged_in: bool,
    client_id: String,
    client_secret: String,

    // Application State
    menu: Menu,
    input_text: String,
    last_operator: Operator

    // Application Data
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            logged_in: false,
            client_id: String::with_capacity(64),
            client_secret: String::with_capacity(64),

            menu: Menu::Login,
            input_text: String::new(),
            last_operator: Operator::None
        }
    }
    fn get_oauth2(&mut self) {
        
    }
}

pub fn main_loop<B: Backend>(terminal: &mut Terminal<B>, app_state: &mut AppState) -> Result<(), std::io::Error>{
    loop {
        terminal.draw(|f| frontend::main_ui(f, app_state));

        let event: Event = event::read()?;
        match event{
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Paste(_) => unimplemented!(),
            Event::Resize(_, _) => todo!(),
            Event::Key(e) => {
                if e.code == KeyCode::Char('q') {break}
                if e.code == KeyCode::Enter {app_state.last_operator = Operator::Enter}
                if e.code == KeyCode::Backspace {app_state.input_text.pop();}
                if let KeyCode::Char(c) = e.code {app_state.input_text.push(c)}
            },
            Event::Mouse(_) => todo!(),
        }
    }
    Ok(())
}