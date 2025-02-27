use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self},
};
use git2::Repository;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::env;
use std::io;
use std::process::Command;
enum MenuItem {
    UpdatePackages,
    CloneRepo,
    UpgradePackages,
    InstallPackages,
    Quit,
}

struct App {
    menu_state: usize,
    menu_items: Vec<(&'static str, MenuItem)>,
    output: String,
    log_messages: Vec<String>,
    scroll_offset: u16,
}

impl App {
    fn new() -> Self {
        Self {
            menu_state: 0,
            menu_items: vec![
                ("Update Packages", MenuItem::UpdatePackages),
                ("Upgrade Packages", MenuItem::UpgradePackages),
                ("Install Packages", MenuItem::InstallPackages),
                ("Clone Repository", MenuItem::CloneRepo),
                ("Quit", MenuItem::Quit),
            ],
            output: String::from("Welcome! Select an option and press Enter to execute."),
            log_messages: Vec::new(),
            scroll_offset: 0,
        }
    }

    fn log_message(&mut self, message: &str) {
        self.log_messages.push(message.to_string());
        if self.log_messages.len() > 100 {
            self.log_messages.remove(0);
        }
    }
    fn next(&mut self) {
        self.menu_state = (self.menu_state + 1) % self.menu_items.len();
    }

    fn previous(&mut self) {
        if self.menu_state > 0 {
            self.menu_state -= 1;
        } else {
            self.menu_state = self.menu_items.len() - 1;
        }
    }

    fn execute_current(&mut self) {
        match self.menu_items[self.menu_state].1 {
            MenuItem::UpdatePackages => self.update_pkgs(),
            MenuItem::CloneRepo => self.clone_repository(),
            MenuItem::UpgradePackages => self.upgrade_packages(),
            MenuItem::InstallPackages => self.install_packages(),
            MenuItem::Quit => {}
        }
    }
    fn get_package_manager(&mut self) -> String {
        let mut package_manager = String::new();
        let apt_check = Command::new("dpkg").arg("--version").output();
        let pacman_check = Command::new("pacman").arg("--version").output();
        let yay_check = Command::new("yay").arg("--version").output();
        match apt_check {
            Ok(output) if output.status.success() => package_manager = String::from("apt"),
            _ => match yay_check {
                Ok(output) if output.status.success() => package_manager = String::from("yay"),
                _ => match pacman_check {
                    Ok(output) if output.status.success() => {
                        package_manager = String::from("pacman")
                    }
                    _ => {
                        panic!("No valid package manager found!");
                    }
                },
            },
        }
        package_manager
    }

    fn update_pkgs(&mut self) {
        let package_manager = self.get_package_manager();
        let output;
        match package_manager.as_str() {
            "apt" => {
                output = Command::new("sudo")
                    .arg("apt")
                    .arg("update")
                    .arg("-y")
                    .output();
            }
            "yay" => {
                output = Command::new("yay").arg("-Sy").output();
            }
            "pacman" => {
                output = Command::new("sudo").arg("pacman").arg("-Sy").output();
            }
            _ => panic!("error occurred"),
        }

        match output {
            Ok(_) => {
                self.output = String::from(format!(
                    "Packages updated successfully with {}!",
                    package_manager
                ));
            }
            Err(e) => self.output = String::from(format!("{}", e)),
        }
    }

    fn clone_repo(&mut self) -> String {
        let repo_url = String::from("https://github.com/farukerdem34/dotfiles.git");
        let home_folder = env::var("HOME").expect("$HOME envirenment variable is not set!");
        let clone_path = format!("{}/.dotfiles", &home_folder);

        match Repository::clone(&repo_url, &clone_path) {
            Ok(_) => String::from("Repository cloned successfully!"),
            Err(e) => e.to_string(),
        }
    }
    fn clone_repository(&mut self) {
        self.output = String::from("Cloning repository...");
        let output = self.clone_repo();
        self.output = String::from(format!("{}", output));
    }

    fn upgrade_packages(&mut self) {
        let package_manager = self.get_package_manager();
        match package_manager.as_str() {
            "apt" => {
                let output = Command::new("sudo")
                    .arg("apt")
                    .arg("upgrade")
                    .arg("-y")
                    .output();
                match output {
                    Ok(_) => {
                        self.output = String::from("Packages upgraded successfully!");
                    }
                    Err(e) => self.output = String::from(format!("{}", e)),
                }
            }
            "yay" => {
                let output = Command::new("yay").arg("-Syu").output();
                match output {
                    Ok(_) => {
                        self.output = String::from("Packages upgraded successfully!");
                    }
                    Err(e) => self.output = String::from(format!("{}", e)),
                }
            }
            "pacman" => {
                let output = Command::new("sudo").arg("pacman").arg("-Syu").output();
                match output {
                    Ok(_) => {
                        self.output = String::from("Packages upgraded successfully!");
                    }
                    Err(e) => self.output = String::from(format!("{}", e)),
                }
            }
            _ => self.output = String::from("No valid package manager found!"),
        }
        self.output = String::from("Function Three executed!");
    }
    fn install_packages(&mut self) {
        let package_manager = self.get_package_manager();
        let packages = vec![
            "bash",
            "btop",
            "fastfetch",
            "kitty",
            "nvim",
            "starship",
            "tmux",
            "vim",
            "zsh",
            "zoxide",
            "stow",
        ];

        let aur_packages = vec!["bat", "fzf", "starship"];
        let _result = match package_manager.as_str() {
            "apt" => {
                let mut command = Command::new("sudo");
                command.arg("apt").arg("install").arg("-y");

                let mut starship_cmd = Command::new("curl");
                starship_cmd
                    .arg("-fsSL")
                    .arg("https://starship.rs/install.sh");
                starship_cmd.arg("-o").arg("/tmp/starship.sh");
                let _ = starship_cmd.spawn();

                let mut install_starship = Command::new("bash");
                install_starship.arg("/tmp/starship.sh");
                let _ = install_starship.spawn();

                for package in packages {
                    command.arg(package);
                }

                let _ = command.output();
                self.output = String::from("Packages Installed!");
            }
            "yay" => {
                let mut command = Command::new("yay");
                command.arg("-S").arg("--noconfirm");

                for package in packages {
                    command.arg(package);
                }

                for aur_package in aur_packages {
                    command.arg(aur_package);
                }
                let _ = command.output();
                self.output = String::from("Packages Installed!");
            }
            "pacman" => {
                let mut command = Command::new("sudo");
                command.arg("pacman").arg("-S").arg("--noconfirm");

                for package in packages {
                    command.arg(package);
                }
                for aur_package in aur_packages {
                    command.arg(aur_package);
                }
                let _ = command.spawn();
                self.output = String::from("Packages Installed!");
            }
            _ => self.output = String::from("No valid package manager found!"),
        };
    }
}

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

fn run_app<B: ratatui::backend::Backend>(
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
                        Constraint::Length(6),
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
