use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use std::{io, thread, time::Duration};

fn main() -> io::Result<()> {
    // Setup terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let menu_items = vec!["Say Hello", "Print Time", "Exit"];
    let mut selected = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(100)])
                .split(size);

            let items: Vec<ListItem> = menu_items
                .iter()
                .enumerate()
                .map(|(i, &item)| {
                    let content = if i == selected {
                        format!("> {}", item) // Highlight selected item
                    } else {
                        format!("  {}", item)
                    };
                    ListItem::new(content)
                })
                .collect();

            let list = List::new(items).block(Block::default().title("Menu").borders(Borders::ALL));

            f.render_widget(list, chunks[0]);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < menu_items.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        match selected {
                            0 => say_hello(),
                            1 => print_time(),
                            2 => break, // Exit app
                            _ => {}
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Esc => break, // Quit app
                    _ => {}
                }
            }
        }
    }

    // Cleanup terminal
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

// Functions executed by menu items
fn say_hello() {
    println!("Hello, World!");
    thread::sleep(Duration::from_secs(1));
}

fn print_time() {
    let now = chrono::Local::now();
    println!("Current Time: {}", now.format("%Y-%m-%d %H:%M:%S"));
    thread::sleep(Duration::from_secs(1));
}
