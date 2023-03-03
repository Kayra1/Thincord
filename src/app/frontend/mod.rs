
use tui::{Frame, backend::{Backend}, layout::{Layout, Direction, Constraint, Rect}, widgets::{Block, Borders, BorderType}};
use crate::app::backend::AppState;

mod login;

pub(super) fn main_ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    // Render any modals on top of the UI
    if !app_state.logged_in() {
        login::login_ui(f, app_state);
        return;
    }

    // Render the UI
    let top_view: Vec<Rect> = Layout::default()
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
    f.render_widget(contacts_block, top_view[0]);

    let messages_block = Block::default()
        .title("Messages")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(messages_block, top_view[1]);

}
