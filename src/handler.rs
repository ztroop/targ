use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

/// Handle key events and update the app state accordingly.
pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    match key_event.code {
        KeyCode::Char('q') => {
            app.quit();
        }
        _ => {}
    }
    Ok(())
}
