use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        KeyCode::Enter | KeyCode::Right => app.enter_directory(),
        KeyCode::Backspace | KeyCode::Left => app.go_back(),
        _ => {}
    }
    Ok(())
}
