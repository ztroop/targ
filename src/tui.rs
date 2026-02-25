use crate::app::App;
use crate::event::EventHandler;
use crate::ui;
use crossterm::cursor::MoveTo;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;
use std::panic;

/// Representation of a terminal user interface.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub events: EventHandler,
}

impl<B: Backend> Tui<B>
where
    B::Error: 'static,
{
    /// Create a new terminal user interface.
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // Define a custom panic hook to reset the terminal properties.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("Failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// Draws the terminal interface.
    pub fn draw(&mut self, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    /// Resets the terminal interface.
    fn reset() -> Result<(), Box<dyn std::error::Error>> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;
        Ok(())
    }

    /// Exits the terminal interface.
    pub fn exit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
