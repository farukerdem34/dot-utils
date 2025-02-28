use crate::utils::App;
use crate::utils::MenuItem;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
pub fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(app.menu_items.len() as u16 + 2),
                        Constraint::Percentage(50),
                        Constraint::Min(5),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let items: Vec<ListItem> = app
                .menu_items
                .iter()
                .enumerate()
                .map(|(i, (name, _))| {
                    let style = if i == app.menu_state {
                        Style::default().fg(Color::Black).bg(Color::White)
                    } else {
                        Style::default().fg(Color::White)
                    };

                    let content = Line::from(vec![Span::styled(name.to_string(), style)]);
                    ListItem::new(content)
                })
                .collect();

            let menu = List::new(items)
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::White)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );

            let output = Paragraph::new(Line::from(app.output.as_str()))
                .block(Block::default().title("Output").borders(Borders::ALL));

            f.render_widget(menu, chunks[0]);
            f.render_widget(output, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                KeyCode::Enter => {
                    if let MenuItem::Quit = app.menu_items[app.menu_state].1 {
                        return Ok(());
                    }
                    app.execute_current();
                }
                _ => {}
            }
        }
    }
}
