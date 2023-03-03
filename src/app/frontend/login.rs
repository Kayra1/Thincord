
use tui::{backend::{Backend}, Frame, layout::{Layout, Direction, Constraint, Alignment}, widgets::{Block, Borders, BorderType, Paragraph, Wrap}, text::{Spans, Span}, style::{Style, Modifier}};
use crate::app::{backend::{AppState, Operator} };


pub(super) fn login_ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let top_view = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(30),
            ].as_ref()
        )
        .split(f.size());
    
    // let login_modal = Block::default()
    //     .title("Log In")
    //     .title_alignment(tui::layout::Alignment::Center)
    //     .borders(Borders::ALL)
    //     .border_type(BorderType::Rounded);

    // f.render_widget(login_modal, top_view[1]);

    // Get Client ID State
    if !app_state.client_id_is_valid() && app_state.last_operator() == &Operator::Enter {
        app_state.save_client_id();
    }
    let id_text = if app_state.client_id_is_valid() {
            vec![
            Spans::from(vec![
                Span::raw("Client ID: "),
                Span::styled(app_state.client_id(), Style::default().add_modifier(Modifier::ITALIC))])
            ]
        } else {
            vec![
            Spans::from(vec![
                Span::raw("Please enter or paste your Client ID: "),
                Span::styled(app_state.input_text(), Style::default().add_modifier(Modifier::ITALIC))
                ]
            )]
        };
    let client_id = Paragraph::new(id_text)
        .block(Block::default().title("Login").borders(Borders::ALL))
        .style(Style::default())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true }); 
    f.render_widget(client_id, top_view[1]);

    // Get Client ID State
    if !app_state.client_secret_is_valid() && app_state.last_operator() == &Operator::Enter {
        app_state.save_client_secret();
    }

    let secret_text = if app_state.client_secret_is_valid() {
            vec![
            Spans::from(vec![
                Span::raw("Client Secret: "),
                Span::styled(app_state.client_secret(), Style::default().add_modifier(Modifier::ITALIC))])
            ]
        } else {
            vec![
            Spans::from(vec![
                Span::raw("Please enter or paste your Client Secret: "),
                Span::styled(app_state.input_text(), Style::default().add_modifier(Modifier::ITALIC))
                ]
            )]
        };
    let client_secret = Paragraph::new(secret_text)
        .block(Block::default().title("Login").borders(Borders::ALL))
        .style(Style::default())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true }); 
    f.render_widget(client_secret, top_view[2]);

}