use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let table = Table::new(
        app.tar_contents.clone(),
        [
            Constraint::Percentage(50),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
    .block(Block::default().title("TAR Contents").borders(Borders::ALL))
    .header(
        Row::new(vec!["File Path", "File Size", "Last Modified"])
            .style(Style::default().fg(Color::Yellow)),
    )
    .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let size = frame.size();
    frame.render_widget(table, size);
}
