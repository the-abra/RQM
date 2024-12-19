mod vm_management;
mod create_vm;
mod logs;
mod network_settings;
mod snapshot_manager;
mod arg;

use clap::Parser;
use rust_i18n::t;
use ratatui::{
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color, Modifier},
    text::{Span, Text},
    prelude::Line,
    Terminal,
    backend::CrosstermBackend
};
use crossterm::event::{self, Event, KeyCode};
use std::io;

rust_i18n::i18n!("locales", fallback = "en");

/// Localization function
pub fn t(key: &str) -> String {
    t!(key).to_string()
}

/// Define application states
#[derive(PartialEq)]
pub enum AppState {
    VMManagement,
    CreateVM,
    Logs,
    NetworkSettings,
    SnapshotManager,
}

/// Render the main menu
fn render_main_menu(selected: usize) -> Paragraph<'static> {
    let title = t("Main Menu");
    let menu_items = [
        t("VM Management"),
        t("Create VM"),
        t("Logs"),
        t("Network Conf"),
        t("Snapshots"),
    ];
    
    let menu_text: Vec<Line> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            if i == selected {
                Line::from(vec![Span::styled(format!("> {}", item), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))])
            } else {
                Line::from(vec![Span::raw(format!("  {}", item))])
            }
        })
        .collect();

    let text = Text::from(menu_text);

    Paragraph::new(text)
        .block(Block::default().title(Span::styled(title, Style::default().fg(Color::Rgb(255, 165, 0)))).borders(Borders::ALL).border_style(Style::default().fg(Color::Rgb(255, 165, 0))))
}

/// Render current page based on the application state
fn render_current_page(state: &AppState) -> Paragraph<'static> {
    let block_style = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green)); // Green color for the border

    match state {
        AppState::VMManagement => vm_management::render().block(block_style),
        AppState::CreateVM => create_vm::render().block(block_style),
        AppState::Logs => logs::render().block(block_style),
        AppState::NetworkSettings => network_settings::render().block(block_style),
        AppState::SnapshotManager => snapshot_manager::render().block(block_style),
    }
}

/// Handle user input
fn handle_input(state: &mut AppState, selected: &mut usize, menu_length: usize) -> bool {
    if let Event::Key(key) = event::read().unwrap() {
        match key.code {
            KeyCode::Up => {
                if *selected > 0 {
                    *selected -= 1;
                }
            }
            KeyCode::Down => {
                if *selected < menu_length - 1 {
                    *selected += 1;
                }
            }
            KeyCode::Enter => match *selected {
                0 => *state = AppState::VMManagement,
                1 => *state = AppState::CreateVM,
                2 => *state = AppState::Logs,
                3 => *state = AppState::NetworkSettings,
                4 => *state = AppState::SnapshotManager,
                _ => {}
            },
            KeyCode::Char('q') => return true,
            KeyCode::Esc => *state = AppState::VMManagement, // Every ESC returns to VMManagement menu
            _ => {}
        }
    }
    false
}

/// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = arg::Args::parse(); // Handle parameters and arguments
    rust_i18n::set_locale(&opt.lang); // Set language for localization

    // Initialize terminal
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Store application state
    let mut app_state = AppState::VMManagement;
    let mut selected_menu = 0;
    let menu_length = 5; // Number of menu items

    // Enter alternate screen and enable raw mode
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

    loop {
        terminal.draw(|frame| {
            // Define layout with two columns
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30), // Left column for menu
                    Constraint::Percentage(70), // Right column for content
                ])
                .split(frame.area());

            // Render main menu on the left
            frame.render_widget(render_main_menu(selected_menu), chunks[0]);

            // Render selected page content on the right
            frame.render_widget(render_current_page(&app_state), chunks[1]);
        })?;

        if handle_input(&mut app_state, &mut selected_menu, menu_length) {
            break; // Exit loop if 'q' is pressed
        }
    }

    // Restore terminal state
    crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    terminal.clear()?;

    Ok(())
}