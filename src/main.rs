mod audio;
mod timer;
mod ui;
mod utils;
use std::{
    io::{self, stdout},
    time::Duration,
};

use crossterm::{
    event::{poll, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::{audio::play_notes, timer::PomodoroTimer, ui::draw_frame};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut pomo = PomodoroTimer::new();

    loop {
        pomo.tick();
        terminal.draw(|frame| {
            draw_frame(frame, &pomo);
        })?;

        if poll(Duration::from_millis(16))? {
            if let Event::Key(key) = crossterm::event::read()? {
                if key.code == KeyCode::Char('q') {
                    play_notes(&[(440.0, 0.25)]);
                    break;
                } else if key.code == KeyCode::Char('p') {
                    pomo.toggle_paused();
                } else if key.code == KeyCode::Char('j') {
                    pomo.skip();
                } else if key.code == KeyCode::Char('r') {
                    pomo.reset();
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
