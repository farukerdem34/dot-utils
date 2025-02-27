use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::{io, thread, time::Duration};

struct App {
    menu_items: Vec<(&'static str, fn() -> String)>,
    menu_state: usize,
    output: String,
}

impl App {
    fn new() -> Self {
        Self {
            menu_items: vec![
                ("Say Hello", say_hello),
                ("Print Time", print_time),
                ("Exit", || "Exiting...".to_string()),
            ],
            menu_state: 0,
            output: "Select an option and press Enter.".to_string(),
        }
    }

    fn next(&mut self) {
        if self.menu_state < self.menu_items.len() - 1 {
            self.menu_state += 1;
        }
    }

    fn previous(&mut self) {
        if self.menu_state > 0 {
            self.menu_state -= 1;
        }
    }

    fn execute_current(&mut self) {
        self.output = (self.menu_items[self.menu_state].1)();
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(6), // Menü
                    Constraint::Min(1),    // Çıktı alanı
                ])
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

            let output = Paragraph::new(Line::from(app.output.as_str()));
            f.render_widget(menu, chunks[0]);
            f.render_widget(output, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                KeyCode::Enter => {
                    if app.menu_state == app.menu_items.len() - 1 {
                        break; // Exit seçildiyse çık
                    }
                    app.execute_current();
                }
                _ => {}
            }
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

// Menüde çalıştırılacak fonksiyonlar
fn say_hello() -> String {
    "Hello, World!".to_string()
}

fn print_time() -> String {
    let now = chrono::Local::now();
    format!("Current Time: {}", now.format("%Y-%m-%d %H:%M:%S"))
}
