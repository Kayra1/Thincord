use crossterm::event::{self, Event, KeyCode};
use tui::{backend::{Backend}, Terminal, Frame, layout::{Layout, Direction, Constraint, Rect}, widgets::{Block, Borders, BorderType}};

mod login;
use crate::thincord::login::login_ui;

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

pub struct State {
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

impl State {
    pub fn new() -> State {
        State {
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

pub fn start<B: Backend>(terminal: &mut Terminal<B>, app_state: &mut State) -> Result<(), std::io::Error>{
    loop {
        terminal.draw(|f| main_ui(f, app_state));

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

fn main_ui<B: Backend>(f: &mut Frame<B>, app_state: &mut State) {
    if !app_state.logged_in {
        login_ui(f, app_state);
        return;
    }
    let parent_chunk: Vec<Rect> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ].as_ref()
        )
        .split(f.size());
    
    let contacts_block = Block::default()
        .title("Contacts")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(contacts_block, parent_chunk[0]);

    let messages_block = Block::default()
        .title("Messages")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(messages_block, parent_chunk[1]);

}

