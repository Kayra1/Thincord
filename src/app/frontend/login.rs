
use tui::{backend::{Backend}, Frame, layout::{Layout, Direction, Constraint, Alignment}, widgets::{Block, Borders, BorderType, Paragraph, Wrap}, text::{Spans, Span}, style::{Style, Modifier}};
use crate::app::backend::{AppState, Operator};


pub(super) fn login_ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let parent_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(70),
            ].as_ref()
        )
        .split(f.size());
    
    let login_modal = Block::default()
        .title("Log In")
        .title_alignment(tui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    f.render_widget(login_modal, parent_chunk[0]);

    // Get Client ID State
    if app_state.client_id() == "" && app_state.last_operator() == &Operator::Enter {
        app_state.set_client_id(app_state.input_text().to_string());
        app_state.input_text_mut().clear();
        app_state.set_last_operator(Operator::None);
    }
    let mut text = vec![
        Spans::from(vec![
            Span::raw("Please enter or paste your Client ID: "),
            Span::styled(app_state.input_text(),Style::default().add_modifier(Modifier::ITALIC))]
        )];
    if app_state.client_id() != "" {
        text = vec![
            Spans::from(vec![
                Span::raw("Client ID: "),
                Span::styled(app_state.client_id() ,Style::default().add_modifier(Modifier::ITALIC))]
            )];
        }
    let client_id = Paragraph::new(text)
        .block(Block::default().title("Login").borders(Borders::ALL))
        .style(Style::default())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true }); 
    f.render_widget(client_id, parent_chunk[0]);

}