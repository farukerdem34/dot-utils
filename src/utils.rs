use git2::Repository;
use std::env;

use std::process::Command;
pub struct App {
    pub menu_state: usize,
    pub menu_items: Vec<(&'static str, MenuItem)>,
    pub output: String,
}

pub enum MenuItem {
    UpdatePackages,
    CloneRepo,
    UpgradePackages,
    InstallPackages,
    Quit,
}

impl App {
    pub fn new() -> Self {
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
        }
    }

    pub fn next(&mut self) {
        self.menu_state = (self.menu_state + 1) % self.menu_items.len();
    }

    pub fn previous(&mut self) {
        if self.menu_state > 0 {
            self.menu_state -= 1;
        } else {
            self.menu_state = self.menu_items.len() - 1;
        }
    }

    pub fn execute_current(&mut self) {
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
        self.output = String::from("Packages upgraded successfully!");
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
