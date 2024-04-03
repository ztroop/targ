use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::app::App;

/// Render the UI.
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(frame.size());

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

    frame.render_stateful_widget(table, chunks[0], &mut app.table_state);

    let hint_rows = vec![Row::new(vec![
        "[q → exit]".to_string(),
        "[up/down → navigate]".to_string(),
    ])
    .style(Style::default().fg(Color::DarkGray))];
    let hint_table =
        Table::new(hint_rows, [Constraint::Length(10), Constraint::Length(20)]).column_spacing(1);

    frame.render_widget(hint_table, chunks[1])
}
