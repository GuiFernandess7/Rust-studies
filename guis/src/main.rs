use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal, Color, Style},
    widgets::{Block, Paragraph, Widget}, Frame,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            let ascii_title_area = ratatui::layout::Rect {
                x: 0,
                y: 0,
                width: area.width,
                height: 8,
            };
            frame.render_widget(
                Paragraph::new(
                    r#"
_____              _
|  _ \(_)         | |
| |_) |_  ___  ___| |_ _ __ ___  __ _ _ __ ___
|  _ <| |/ _ \/ __| __| '__/ _ \/ _` | '_ ` _ \
| |_) | | (_) \__ \ |_| | |  __/ (_| | | | | | |
|____/|_|\___/|___/\__|_|  \___|\__,_|_| |_| |_|
        "#
            )
            .green()
            .on_black()
            .alignment(ratatui::layout::Alignment::Left)
            .bold(),
            ascii_title_area,
            );

            let paragraph_area = ratatui::layout::Rect {
                x: 0,
                y: 9,
                width: area.width,
                height: area.height - 9,
            };
            frame.render_widget(
                Paragraph::new("A Client for Sending Genetic Streamin. Based on Websocket Communication.")
                    .green()
                    .on_black()
                    .alignment(ratatui::layout::Alignment::Left),
                paragraph_area,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
