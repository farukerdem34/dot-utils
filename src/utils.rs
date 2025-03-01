use git2::Repository;
use std::env;
use std::path::Path;
use std::process::Command;

pub struct App {
    pub menu_state: usize,
    pub menu_items: Vec<(&'static str, MenuItem)>,
    pub output: String,
    packages: Vec<&'static str>,
    aur_packages: Vec<&'static str>,
    pub neovim_menu_items: Vec<(&'static str, NeoVimItem)>,
    pub neovim_menu_state: usize,
    pub is_in_neovim_menu: bool,
}

pub enum MenuItem {
    UpdatePackages,
    CloneRepo,
    UpgradePackages,
    InstallPackages,
    LinkDotFiles,
    UnLinkDotFiles,
    SyncDotFiles,
    NeoVimMenu,
    Quit,
}

pub enum NeoVimItem {
    BackUpState,
    BackUpShare,
    BackupCache,
    BackToMainMenu,
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
                ("Link Dotfiles", MenuItem::LinkDotFiles),
                ("Unlink Dotfiles", MenuItem::UnLinkDotFiles),
                ("Sync Dotfiles", MenuItem::SyncDotFiles),
                ("NeoVim",MenuItem::NeoVimMenu),
                ("Quit", MenuItem::Quit),
            ],
            output: String::from("Welcome! Select an option and press Enter to execute."),
            packages: vec![
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
            ],
            aur_packages: vec!["bat", "fzf", "starship"],
            neovim_menu_items: vec![
                ("Back Up State", NeoVimItem::BackUpState),
                ("Back Up Share", NeoVimItem::BackUpShare),
                ("Back Up Cache", NeoVimItem::BackupCache),
                ("Back To Main Menu", NeoVimItem::BackToMainMenu),
            ],
            neovim_menu_state: 0,
            is_in_neovim_menu: false,
        }
    }

    pub fn next(&mut self) {
        if self.is_in_neovim_menu {
            self.neovim_menu_state = (self.neovim_menu_state + 1) % self.neovim_menu_items.len();
        }else{

        self.menu_state = (self.menu_state + 1) % self.menu_items.len();
        }
    }

    pub fn previous(&mut self) {
        if self.is_in_neovim_menu {
    if self.neovim_menu_state > 0 {
                self.neovim_menu_state -= 1;
            } else {
                self.neovim_menu_state = self.neovim_menu_items.len() - 1;
            }
    
        }else{
        if self.menu_state > 0 {
            self.menu_state -= 1;
        } else {
            self.menu_state = self.menu_items.len() - 1;
        }
        }
    }

    pub fn execute_current(&mut self) {
        if self.is_in_neovim_menu {
            match self.neovim_menu_items[self.neovim_menu_state].1 {
                NeoVimItem::BackUpState => self.backup_state(),
                NeoVimItem::BackUpShare => self.backup_share(),
                NeoVimItem::BackupCache => self.backup_cache(),
                NeoVimItem::BackToMainMenu => self.is_in_neovim_menu = false,
            }
        }else{
                    match self.menu_items[self.menu_state].1 {
            MenuItem::UpdatePackages => self.update_pkgs(),
            MenuItem::CloneRepo => self.clone_repository(),
            MenuItem::UpgradePackages => self.upgrade_packages(),
            MenuItem::InstallPackages => self.install_packages(),
            MenuItem::LinkDotFiles => self.link_dot_files(),
            MenuItem::UnLinkDotFiles => self.unstow_dot_files(),
            MenuItem::SyncDotFiles => self.update_dotfiles(),
            MenuItem::Quit => {}
            MenuItem::NeoVimMenu => {
                self.is_in_neovim_menu = true;
                self.output = String::from("Welcome to NeoVim Menu!");
            }
        }

        }
    }
// TO DO 
    fn backup_share(&mut self) {
    }

    fn backup_state(&mut self) {
    }

    fn backup_cache(&mut self) {
    }
    fn is_command_exist(&mut self, cmd: &str, checker: Option<&str>) -> bool {
        let checker = checker.unwrap_or("--version");
        let output = Command::new(&cmd).arg(&checker).output();
        let is_exist;
        match output {
            Ok(_) => is_exist = true,
            _ => is_exist = false,
        }
        is_exist
    }

    fn get_package_manager(&mut self) -> String {
        let package_manager;
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
    fn get_home_directory(&mut self) -> String {
        let home_folder = env::var("HOME").expect("$HOME envirenment variable is not set!");
        home_folder
    }
    fn clone_repo(&mut self) -> String {
        let repo_url = String::from("https://github.com/farukerdem34/dotfiles.git");
        let home_folder = self.get_home_directory();
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

                for package in &self.packages {
                    command.arg(package);
                }

                let _ = command.output();
                self.output = String::from("Packages Installed!");
            }
            "yay" => {
                let mut command = Command::new("yay");
                command.arg("-S").arg("--noconfirm");

                for package in &self.packages {
                    command.arg(package);
                }

                for aur_package in &self.aur_packages {
                    command.arg(aur_package);
                }
                let _ = command.output();
                self.output = String::from("Packages Installed!");
            }
            "pacman" => {
                let mut command = Command::new("sudo");
                command.arg("pacman").arg("-S").arg("--noconfirm");

                for package in &self.packages {
                    command.arg(package);
                }
                for aur_package in &self.aur_packages {
                    command.arg(aur_package);
                }
                let _ = command.spawn();
                self.output = String::from("Packages Installed!");
            }
            _ => self.output = String::from("No valid package manager found!"),
        };
    }
    fn link_dot_files(&mut self) {
        if self.is_command_exist("stow", Some("--version")) {
            let home = self.get_home_directory();
            let work_path;
            if Path::new(format!("{}/dotfiles", home).as_str()).is_dir() {
                work_path = format!("{}/dotfiles", home);
            } else if Path::new(format!("{}/dotfiles", home).as_str()).is_dir() {
                work_path = format!("{}/.dotfiles", home);
            } else {
                self.output = String::from("Dotfiles directory not found in $HOME/.dotfiles");
                panic!("Dotfiles directory not found in $HOME/.dotfiles");
            }

            let mut cmd = Command::new("stow");
            cmd.current_dir(work_path);
            let packages = vec![
                "bash",
                "bat",
                "btop",
                "fastfetch",
                "kitty",
                "nvim",
                "starship",
                "tmux",
                "vimrc",
                "zsh",
            ];
            for pkg in packages {
                cmd.arg(pkg);
            }
            let output = cmd.output();
            match output {
                Ok(_) => self.output = String::from("Packages stowed succesfully."),
                Err(e) => self.output = format!("{}", e),
            }
        } else {
            self.output = String::from("Stow package doesn't exists.")
        }
    }

    fn unstow_dot_files(&mut self) {
        // First, check if stow is installed
        if !self.is_command_exist("stow", None) {
            self.output = String::from("stow is not installed. Install it first.");
            return;
        }

        // Get home directory and check if .dotfiles directory exists
        let home_folder = self.get_home_directory();
        let dotfiles_path = format!("{}/.dotfiles", &home_folder);

        // Check if .dotfiles directory exists
        let dotfiles_dir = std::path::Path::new(&dotfiles_path);
        if !dotfiles_dir.exists() {
            self.output = String::from("Dotfiles directory not found. Clone the repository first.");
            return;
        }

        // Unstow each directory in .dotfiles
        let dotfiles_contents = match std::fs::read_dir(&dotfiles_path) {
            Ok(entries) => entries,
            Err(e) => {
                self.output = format!("Failed to read dotfiles directory: {}", e);
                return;
            }
        };

        let mut success_count = 0;
        let mut error_messages = Vec::new();

        for entry in dotfiles_contents {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let dir_name = entry.file_name();
                        if let Some(dir_str) = dir_name.to_str() {
                            // Skip .git directory and other hidden directories
                            if dir_str.starts_with('.') {
                                continue;
                            }

                            // Run stow -D command
                            let output = std::process::Command::new("stow")
                                .arg("-D") // Delete flag
                                .arg("-v") // Verbose
                                .arg("-d") // Directory
                                .arg(&dotfiles_path) // Stow dir
                                .arg("-t") // Target
                                .arg(&home_folder) // Target dir
                                .arg(dir_str) // Package name
                                .output();

                            match output {
                                Ok(cmd_output) => {
                                    if cmd_output.status.success() {
                                        success_count += 1;
                                    } else {
                                        let error = String::from_utf8_lossy(&cmd_output.stderr);
                                        error_messages.push(format!(
                                            "Failed to unstow {}: {}",
                                            dir_str, error
                                        ));
                                    }
                                }
                                Err(e) => {
                                    error_messages
                                        .push(format!("Command error for {}: {}", dir_str, e));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Build output message
        if success_count > 0 {
            self.output = format!(
                "Successfully unstowed {} dotfile directories.",
                success_count
            );
            if !error_messages.is_empty() {
                self.output.push_str("\n\nErrors encountered:");
                for msg in error_messages {
                    self.output.push_str(&format!("\n- {}", msg));
                }
            }
        } else if error_messages.is_empty() {
            self.output = String::from("No dotfiles were found to unstow.");
        } else {
            self.output = String::from("Failed to unstow dotfiles. Errors encountered:");
            for msg in error_messages {
                self.output.push_str(&format!("\n- {}", msg));
            }
        }
    }

    fn update_dotfiles(&mut self) {
        self.output = String::from("Updating dotfiles repository...");

        let home_folder = self.get_home_directory();
        let dotfiles_path = format!("{}/.dotfiles", &home_folder);

        // Check if the .dotfiles directory exists
        if !Path::new(&dotfiles_path).exists() {
            self.output =
                String::from("Dotfiles directory not found. Try cloning the repository first.");
            return;
        }

        // Open the repository
        match Repository::open(&dotfiles_path) {
            Ok(repo) => {
                // Get the remote
                match repo.find_remote("origin") {
                    Ok(mut remote) => {
                        // Fetch updates
                        let fetch_result =
                            remote.fetch(&["refs/heads/*:refs/remotes/origin/*"], None, None);
                        if let Err(e) = fetch_result {
                            self.output = format!("Failed to fetch from remote: {}", e);
                            return;
                        }

                        // Get default branch reference
                        let head = match repo.head() {
                            Ok(head) => head,
                            Err(e) => {
                                self.output = format!("Failed to get HEAD reference: {}", e);
                                return;
                            }
                        };

                        // Get current branch name
                        let branch_name = match head.shorthand() {
                            Some(name) => name,
                            _ => {
                                self.output =
                                    String::from("Failed to determine current branch name");
                                return;
                            }
                        };

                        // Lookup the remote branch we want to merge from
                        let remote_branch_name = format!("origin/{}", branch_name);
                        let remote_ref = match repo
                            .find_reference(&format!("refs/remotes/{}", remote_branch_name))
                        {
                            Ok(reference) => reference,
                            Err(e) => {
                                self.output = format!("Failed to find remote reference: {}", e);
                                return;
                            }
                        };

                        // Convert the reference to an annotated commit
                        let annotated_commit = match repo.reference_to_annotated_commit(&remote_ref)
                        {
                            Ok(commit) => commit,
                            Err(e) => {
                                self.output = format!("Failed to get annotated commit: {}", e);
                                return;
                            }
                        };

                        let fetch_commit = match remote_ref.peel_to_commit() {
                            Ok(commit) => commit,
                            Err(e) => {
                                self.output = format!("Failed to get remote commit: {}", e);
                                return;
                            }
                        };

                        // Perform the merge/pull
                        let mut index = match repo.index() {
                            Ok(index) => index,
                            Err(e) => {
                                self.output = format!("Failed to get repository index: {}", e);
                                return;
                            }
                        };

                        // Create merge options
                        let mut merge_options = git2::MergeOptions::new();
                        merge_options.file_favor(git2::FileFavor::Normal);

                        // Merge the remote branch into the current branch
                        if let Err(e) =
                            repo.merge(&[&annotated_commit], Some(&mut merge_options), None)
                        {
                            self.output = format!("Failed to merge updates: {}", e);
                            return;
                        }

                        // Check for merge conflicts
                        if index.has_conflicts() {
                            self.output = String::from(
                                "Merge conflicts detected. Please resolve them manually.",
                            );
                            return;
                        }

                        // Commit the merge if necessary
                        let result = repo.state();
                        if result == git2::RepositoryState::Merge {
                            // We need to commit the merge
                            let signature = match repo.signature() {
                                Ok(sig) => sig,
                                Err(e) => {
                                    self.output = format!("Failed to create signature: {}", e);
                                    return;
                                }
                            };

                            if let Err(e) = index.write() {
                                self.output = format!("Failed to write index: {}", e);
                                return;
                            }

                            let tree_oid = match index.write_tree() {
                                Ok(oid) => oid,
                                Err(e) => {
                                    self.output = format!("Failed to write tree: {}", e);
                                    return;
                                }
                            };

                            let tree = match repo.find_tree(tree_oid) {
                                Ok(tree) => tree,
                                Err(e) => {
                                    self.output = format!("Failed to find tree: {}", e);
                                    return;
                                }
                            };

                            let head_commit =
                                match repo.head().and_then(|head| head.peel_to_commit()) {
                                    Ok(commit) => commit,
                                    Err(e) => {
                                        self.output = format!("Failed to get HEAD commit: {}", e);
                                        return;
                                    }
                                };

                            if let Err(e) = repo.commit(
                                Some("HEAD"),
                                &signature,
                                &signature,
                                "Merge remote changes",
                                &tree,
                                &[&head_commit, &fetch_commit],
                            ) {
                                self.output = format!("Failed to commit merge: {}", e);
                                return;
                            }

                            // Clean up the repository state
                            if let Err(e) = repo.cleanup_state() {
                                self.output = format!("Failed to cleanup repository state: {}", e);
                                return;
                            }
                        }

                        self.output = String::from("Dotfiles repository updated successfully!");
                    }
                    Err(e) => {
                        self.output = format!("Failed to find remote 'origin': {}", e);
                    }
                }
            }
            Err(e) => {
                self.output = format!("Failed to open repository: {}", e);
            }
        }
    }
}
