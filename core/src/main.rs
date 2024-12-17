use rust_i18n::t;
use clap::Parser;

mod arg;

rust_i18n::i18n!("locales", fallback = "en");

pub fn t(key: &str) -> String {
    t!(key).to_string()
// Example call: println!("{}", t("hello world"));
}


use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEvent, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;



fn main() -> io::Result<()> {
	let opt = arg::Args::parse(); // Handle parameters and arguments
	rust_i18n::set_locale(&opt.lang); // Get --lang <lang> parameter here 


	 // Setup terminal
	 enable_raw_mode()?; // Enables raw mode for capturing input
	 let mut stdout = io::stdout();
	 execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
	 let backend = CrosstermBackend::new(stdout);
	 let mut terminal = Terminal::new(backend)?;
 
	 // Application logic
	 let res = run_app(&mut terminal);
 
	 // Restore terminal
	 disable_raw_mode()?;
	 execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
	 terminal.show_cursor()?;
 
	 if let Err(err) = res {
		 eprintln!("Error: {}", err);
	 }
 
	 Ok(())
	
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
) -> io::Result<()> {
    let mut right_block_text = t("Right Block").to_string();

    loop {
        terminal.draw(|frame| {
            // Divide the screen into two horizontal chunks
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.size());

            // Left Block
            let left_block = Block::default()
                .title(Span::styled(t("Left"), Style::default()))
                .title_alignment(ratatui::layout::Alignment::Center)
                .borders(Borders::ALL);
            frame.render_widget(left_block, chunks[0]);

            // Right Block with dynamic text
            let right_block = Paragraph::new(right_block_text.clone())
                .block(Block::default()
                .title(Span::styled(t("Right"), Style::default()))
                .title_alignment(ratatui::layout::Alignment::Center)
                .borders(Borders::ALL));
            frame.render_widget(right_block, chunks[1]);
        })?;

        // Event handling
        if let Event::Mouse(mouse_event) = event::read()? {
            if let MouseEventKind::Down(_) = mouse_event.kind {
                let x = mouse_event.column;
                let y = mouse_event.row;

                // Determine if the click is inside the left block
                if x < terminal.size()?.width / 2 {
                    right_block_text = t("Left Block Clicked!").to_string();
                }
            }
        }

        // Handle quitting on 'q' key
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    Ok(())
}
