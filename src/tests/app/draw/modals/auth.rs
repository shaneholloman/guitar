use super::*;
use crate::git::auth::{AuthChallenge, AuthProtocol};
use ratatui::{Terminal, backend::TestBackend};

#[test]
fn auth_modal_masks_secret_input() {
    let mut app = App {
        pending_auth_prompt: Some(AuthChallenge {
            url: "https://github.com/asinglebit/guitar.git".to_string(),
            username: Some("octo".to_string()),
            protocol: AuthProtocol::Https,
            operation: "Fetch".to_string(),
            key_path: None,
        }),
        auth_input_field: AuthInputField::Secret,
        ..Default::default()
    };
    app.auth_username_input.set_value("octo");
    app.auth_secret_input.set_value("supersecret");

    let backend = TestBackend::new(90, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw_modal_auth(frame)).unwrap();

    let rendered = terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect::<String>();
    assert!(!rendered.contains("supersecret"));
    assert!(rendered.contains("***********"));
}
