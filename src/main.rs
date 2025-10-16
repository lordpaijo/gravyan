mod app;
mod colors;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use app::{App, AppState};
use ui::ui;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.state {
                AppState::Input => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                    KeyCode::Enter => app.calculate(),
                    KeyCode::Left | KeyCode::Right | KeyCode::Tab => app.switch_field(),
                    KeyCode::Backspace => app.handle_backspace(),
                    KeyCode::Char(c) => app.handle_input(c),
                    _ => {}
                },
                AppState::Result => match key.code {
                    KeyCode::Enter | KeyCode::Char(' ') => app.reset(),
                    KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                    _ => {}
                },
            }
        }
    }
}
