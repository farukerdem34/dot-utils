mod terminal_loop;
mod utils;
use crossterm::{
    execute,
    terminal::{self},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use terminal_loop::run_app;
use utils::App;
fn main() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
