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
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(if app.is_in_neovim_menu {
                            app.neovim_menu_items.len() as u16 + 2
                        } else {
                            app.menu_items.len() as u16 + 2
                        }),
                        Constraint::Percentage(50),
                        Constraint::Min(5),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let menu_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(25), // Left margin
                        Constraint::Percentage(50), // Center area for menu
                        Constraint::Percentage(25), // Right margin
                    ]
                    .as_ref(),
                )
                .split(main_chunks[0]);

            let items: Vec<ListItem> = if app.is_in_neovim_menu {
                // Render submenu items
                app.neovim_menu_items
                    .iter()
                    .enumerate()
                    .map(|(i, (name, _))| {
                        let style = if i == app.neovim_menu_state {
                            Style::default().fg(Color::Black).bg(Color::White)
                        } else {
                            Style::default().fg(Color::White)
                        };

                        let content = Line::from(vec![Span::styled(name.to_string(), style)]);
                        ListItem::new(content)
                    })
                    .collect()
            } else {
                // Render main menu items
                app.menu_items
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
                    .collect()
            };

            let menu_title = if app.is_in_neovim_menu {
                "Advanced Options"
            } else {
                "Menu"
            };

            let menu = List::new(items)
                .block(Block::default().title(menu_title).borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::White)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );

            let output = Paragraph::new(Line::from(app.output.as_str()))
                .block(Block::default().title("Output").borders(Borders::ALL));

            f.render_widget(menu, menu_chunks[1]);
            f.render_widget(output, main_chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    if app.is_in_neovim_menu {
                        app.is_in_neovim_menu = false;
                        app.output = String::from("Returned to main menu. Select an option.");
                    } else {
                        return Ok(());
                    }
                },
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                KeyCode::Enter => {
                    // If in main menu and user selects quit
                    if !app.is_in_neovim_menu && 
                       matches!(app.menu_items[app.menu_state].1, MenuItem::Quit) {
                        return Ok(());
                    }
                    app.execute_current();
                }
                KeyCode::Esc => {
                    if app.is_in_neovim_menu {
                        app.is_in_neovim_menu = false;
                        app.output = String::from("Returned to main menu. Select an option.");
                    }
                }
                _ => {}
            }
        }
    }
}
