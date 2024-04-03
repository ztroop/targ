use crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, structs::FileOrDir};

/// Handle key events and update the app state accordingly.
pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        KeyCode::Char('b') => {
            app.current_path.pop();
        }
        KeyCode::Enter => {
            if let Some(selected) = app.table_state.selected() {
                let contents = app.display_contents();
                if let Some(FileOrDir::Dir { path, .. }) = contents.get(selected).map(|d| *d) {
                    let path_segments: Vec<&str> = path.split('/').collect();
                    if let Some(dir_name) = path_segments.last() {
                        app.current_path.push(dir_name.to_string());
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
