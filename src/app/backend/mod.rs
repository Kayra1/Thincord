pub(super) struct AppState {
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
pub(super) enum Menu {
    Login,
    Main,
    Settings
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Operator {
    None,
    Enter,
    Escape
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

    pub(super) fn logged_in(&self) -> bool {
        self.logged_in
    }

    pub(super) fn client_id(&self) -> &str {
        self.client_id.as_ref()
    }

    pub(super) fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    pub(super) fn client_secret(&self) -> &str {
        self.client_secret.as_ref()
    }

    pub(super) fn input_text(&self) -> &str {
        self.input_text.as_ref()
    }
    
    pub(super) fn input_text_mut(&mut self) -> &mut String {
        &mut self.input_text
    }

    pub(super) fn last_operator(&self) -> &Operator {
        &self.last_operator
    }

    pub(super) fn set_last_operator(&mut self, last_operator: Operator) {
        self.last_operator = last_operator;
    }
}