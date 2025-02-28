# Dot-Utils

![GitHub top language](https://img.shields.io/github/languages/top/farukerdem34/dot-utils?style=flat-square)
![License](https://img.shields.io/github/license/farukerdem34/dot-utils?style=flat-square)- Last updated: 2025-02-28

A terminal-based utility tool written in Rust for managing your dotfiles and system packages across different Linux distributions.

## Features

- **Package Management**
  - Update system packages
  - Upgrade system packages
  - Install predefined essential packages

- **Dotfiles Management**
  - Clone dotfiles repository from GitHub
  - Link dotfiles using GNU Stow
  - Unlink dotfiles when needed
  - Sync and update dotfiles repository

- **Cross-Distribution Support**
  - Automatically detects package manager (apt, pacman, yay)
  - Works on Debian/Ubuntu, Arch Linux and derivatives

- **Terminal UI**
  - Interactive terminal interface using Ratatui and Crossterm
  - Easy-to-navigate menu for all operations

## Prerequisites

- Rust and Cargo
- Git
- Administrative privileges (for package installation)

## Installation

### Standard Build

1. Clone this repository:

   ```bash
   git clone https://github.com/farukerdem34/dot-utils.git
   cd dot-utils
   ```

2. Build with Cargo:

   ```bash
   cargo build --release
   ```

3. Run the binary:

   ```bash
   ./target/release/dot-utils
   ```

4. (Optional) Install the binary to your system:

   ```bash
   cargo install --path .
   ```

### Using Cargo

```bash
cargo install --git https://github.com/farukerdem34/dot-utils.git
```

### Docker Build

1. Clone the repository:

   ```bash
   git clone https://github.com/farukerdem34/dot-utils.git
   cd dot-utils
   ```

2. Build the Docker image:

   ```bash
   docker build -t dot-utils .
   ```

3. Run the container with proper permissions for accessing your home directory:

   ```bash
   docker run -it --rm \
     -v $HOME:/home/user \
     -v /var/run/docker.sock:/var/run/docker.sock \
     --user $(id -u):$(id -g) \
     dot-utils
   ```

#### Docker Compose (Alternative)

Create a `docker-compose.yml` file with the following content:

```yaml
version: '3'
services:
  dot-utils:
    build: .
    volumes:
      - $HOME:/home/user
      - /var/run/docker.sock:/var/run/docker.sock
    user: "${UID}:${GID}"
    tty: true
    stdin_open: true
```

Then run:

```bash
export UID=$(id -u)
export GID=$(id -g)
docker-compose up --build
```

## Usage

Run the application:

```bash
dot-utils
```

### Navigation

- Use **↑** and **↓** arrow keys to navigate through menu items
- Press **Enter** to select and execute an option
- Press **q** or select "Quit" option to exit the application

### Available Options

1. **Update Packages** - Updates package lists from repositories
2. **Upgrade Packages** - Upgrades installed packages to their latest versions
3. **Install Packages** - Installs predefined essential packages
4. **Clone Repository** - Clones the dotfiles repository from GitHub
5. **Link Dotfiles** - Symlinks dotfiles to their appropriate locations using stow
6. **Unlink Dotfiles** - Removes symlinks created by stow
7. **Sync Dotfiles** - Updates dotfiles from the remote repository
8. **Quit** - Exits the application

## Included Packages

### System Packages

- bash
- btop
- fastfetch
- kitty
- nvim
- starship
- tmux
- vim
- zsh
- zoxide
- stow

### AUR Packages (Arch Linux)

- bat
- fzf
- starship

## Customization

To customize the list of packages or dotfiles managed by this utility, modify the following files:

- `src/utils.rs`: Edit the `packages` and `aur_packages` vectors in the `App::new()` implementation

## How It Works

1. **Package Management**:
   - Automatically detects your system's package manager
   - Executes appropriate commands for your distribution

2. **Dotfiles Management**:
   - Uses GNU Stow to create symbolic links
   - Manages dotfiles from `$HOME/.dotfiles` directory
   - Supports standard dotfile organization (one directory per application)

3. **Repository Handling**:
   - Clones dotfiles from <https://github.com/farukerdem34/dotfiles.git>
   - Handles git operations for syncing and updating

## Troubleshooting

### Common Issues

1. **Permission Denied Errors**
   - Ensure you have appropriate permissions for package management operations
   - Use with sudo when necessary

2. **Missing Stow**
   - Install GNU Stow manually if needed: `sudo apt install stow` or `sudo pacman -S stow`

3. **Git Errors**
   - Make sure git is installed and properly configured
   - Check network connectivity for repository operations

4. **Docker Issues**
   - If using Docker, ensure you have mounted your home directory correctly
   - Check that the container has proper permissions to access files

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the terminal interface
- Uses [Crossterm](https://github.com/crossterm-rs/crossterm) for terminal manipulation
- Implements [git2-rs](https://github.com/rust-lang/git2-rs) for Git operations

---

Created by [farukerdem34](https://github.com/farukerdem34)
