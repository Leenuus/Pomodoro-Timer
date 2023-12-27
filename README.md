# TUI Pomodoro Timer

A simple Pomodoro Timer implemented in Rust with [`rata-tui`](https://crates.io/crates/ratatui) and [`crossterm`](https://crates.io/crates/crossterm)


## Progress

### Basic Features

- [X] Basic User Interface
- [X] Timer Controller(Start, Pause, Skip, Break, Long Break)
- [ ] Task List(Add Task, task specific settings, delete Tasks)
- [ ] Timer Presets 
- [ ] Dynamic Keybindings, Vim-like Keybindings
- [ ] Stop Watch
- [ ] Store User Settings and Data
- [ ] Backup and restore user's `settings` and `data`

### ColorScheme

- [ ] Different Colors for Different States(default for Pomodoro, `Green` for short break, `Blue` for long break)

### MultiMedia

- [ ] Talk to `PulseAudio` to play sound
- [ ] Talk to `Dbus` to send notifications

### Advanced Features

- [ ] Generate Reports from using statistics
