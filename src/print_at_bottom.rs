use crossterm::{
    cursor,
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};

fn print_at_bottom(message: &str) -> std::io::Result<()> {
    let mut stdout = stdout();

    // Get terminal size
    let (_, height) = terminal::size()?;

    // Save cursor position
    stdout.execute(cursor::SavePosition)?;

    // Move to bottom line
    stdout.execute(cursor::MoveTo(0, height - 1))?;

    // Clear the line
    stdout.execute(Clear(ClearType::CurrentLine))?;

    // Print the message
    write!(stdout, "{}", message)?;

    // Restore cursor position
    stdout.execute(cursor::RestorePosition)?;

    stdout.flush()?;

    Ok(())
}

// Example usage
fn main() -> std::io::Result<()> {
    // Regular terminal operations...

    // Print a message at the bottom when needed:
    print_at_bottom("This appears at the bottom of the terminal")?;

    // Continue with normal operations...
    Ok(())
}
