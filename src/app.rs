use std::error;

use ratatui::widgets::Row;

pub struct App {
    pub running: bool,
    pub tar_contents: Vec<Row<'static>>,
}

impl App {
    pub fn new(tar_contents: Vec<Row<'static>>) -> Self {
        Self {
            running: true,
            tar_contents,
        }
    }

    pub fn tick(&mut self) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
