use crate::state::{InputMode, State};
use tui::widgets::List;
use tui::widgets::ListItem;
use tui::widgets::Row;
use tui::widgets::Table;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;
const BANNER: &'static str = r#"
                             
                                          
                        _/  _/
     _/_/_/  _/    _/  _/  _/  _/    _/
  _/    _/  _/    _/  _/  _/  _/    _/
 _/    _/  _/    _/  _/  _/  _/    _/
  _/_/_/    _/_/_/  _/  _/    _/_/_/
     _/                          _/
_/_/                        _/_/


               "#;

pub fn search<B>(f: &mut Frame<B>, state: &State)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(5), Constraint::Max(2), Constraint::Max(3)].as_ref())
        .split(f.size());

    let mut index = 0;
    //let list = List::new(
    //    state
    //        .titles
    //        .iter()
    //        .map(|t| {
    //            index += 1;
    //            ListItem::new(Text::raw(format!("{}    {}", index, t)))
    //        })
    //        .collect::<Vec<ListItem>>(),
    //)
    //.block(Block::default().borders(Borders::NONE));

    let rows = state.titles.iter().map(|t| {
        index += 1;
        let row: Vec<String> = vec![index.to_string(), t.clone(), "33:33".into()];
        Row::Data(row.into_iter())
    });
    let table = Table::new(vec!["test", "test", "test"].into_iter(), rows).widths(&[
        Constraint::Min(5),
        Constraint::Min(10),
        Constraint::Max(8),
    ]);

    f.render_widget(table, chunks[0]);

    let msg = match state.input_mode {
        InputMode::Normal => "Press C-c to exit, e to start editing.",
        InputMode::Editing => "Press Esc to stop editing, Enter to record the message",
    };
    let help_message = Paragraph::new(Spans::from(msg));
    f.render_widget(help_message, chunks[1]);

    let input = Paragraph::new(Spans::from(state.input.clone()))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[2]);
    match state.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            //// Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[2].x + state.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[2].y + 1,
            );
        }
    }
}
