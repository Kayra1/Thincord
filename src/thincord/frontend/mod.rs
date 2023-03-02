
use tui::{Frame, backend::{Backend}, layout::{Layout, Direction, Constraint, Rect}, widgets::{Block, Borders, BorderType}};
use crate::AppState;

mod login;

pub fn main_ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    if !app_state.logged_in {
        login::login_ui(f, app_state);
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
