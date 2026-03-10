use std::{
    fmt,
    fs::read,
    io::{self, stdout},
    process::{self, exit},
    time::{Duration, Instant},
};

use crossterm::{
    event::{poll, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use notify_rust::{Notification, Timeout};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use rodio::{source::SineWave, Player, Source};
use tui_big_text::{BigText, PixelSize};

const LONG_DURATION: u32 = 15 * 60;
const SHORT_DURATION: u32 = 5 * 60;
const WORK_DURATION: u32 = 25 * 60;

#[derive(Debug, PartialEq)]
enum PomodoroMode {
    Work,
    ShortBreak,
    LongBreak,
}

impl fmt::Display for PomodoroMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PomodoroMode::Work => write!(f, "Work"),
            PomodoroMode::ShortBreak => write!(f, "Short Break"),
            PomodoroMode::LongBreak => write!(f, "Long Break :)"),
        }
    }
}

struct PomodoroTimer {
    is_paused: bool,
    current_mode: PomodoroMode,
    time_remaining: u32,
    pomodoros_completed: u32,
    time_running: Instant,
    cycles_completed: u32,
}

impl PomodoroTimer {
    fn tick(&mut self) {
        if self.time_running.elapsed().as_secs() >= 1 {
            if self.is_paused {
                return;
            }
            self.time_running = Instant::now();
            if self.time_remaining > 0 {
                self.time_remaining -= 1;
            } else {
                if self.current_mode == PomodoroMode::Work {
                    self.pomodoros_completed += 1;
                    self.cycles_completed += 1;
                }
                let next_mode = self.get_next_mode();
                Notification::new()
                    .summary("Boomodoro")
                    .body(&format!("Mode Change {}", next_mode.to_string()))
                    .icon("🦥")
                    .timeout(Timeout::Milliseconds(6000))
                    .show()
                    .unwrap();
                self.transition(next_mode);
                play_notes(&[(261.0, 0.25), (330.0, 0.25), (261.0, 0.25)]);
            }
        }
    }

    fn toggle_paused(&mut self) {
        play_notes(&[(330.0, 0.25)]);
        self.is_paused = !self.is_paused;
    }

    fn get_next_mode(&self) -> PomodoroMode {
        match self.current_mode {
            PomodoroMode::Work => {
                if self.pomodoros_completed % 4 == 0 {
                    return PomodoroMode::LongBreak;
                } else {
                    return PomodoroMode::ShortBreak;
                }
            }
            PomodoroMode::ShortBreak => return PomodoroMode::Work,
            PomodoroMode::LongBreak => return PomodoroMode::Work,
        }
    }

    fn transition(&mut self, stage: PomodoroMode) {
        self.current_mode = stage;
        match self.current_mode {
            PomodoroMode::Work => self.time_remaining = WORK_DURATION,
            PomodoroMode::LongBreak => {
                self.time_remaining = LONG_DURATION;
                self.cycles_completed = 0;
            }
            PomodoroMode::ShortBreak => self.time_remaining = SHORT_DURATION,
        }
    }

    fn reset(&mut self) {
        self.is_paused = true;
        self.cycles_completed = 0;
        self.current_mode = PomodoroMode::Work;
        self.pomodoros_completed = 0;
        self.time_running = Instant::now();
        self.time_remaining = WORK_DURATION;
    }

    fn skip(&mut self) {
        if self.current_mode == PomodoroMode::Work {
            self.cycles_completed += 1;
            self.pomodoros_completed += 1;
        }
        let next_mode = self.get_next_mode();
        self.transition(next_mode);
        play_notes(&[(261.0, 0.25)]);
    }

    fn new() -> PomodoroTimer {
        PomodoroTimer {
            time_running: Instant::now(),
            is_paused: true,
            cycles_completed: 0,
            current_mode: PomodoroMode::Work,
            time_remaining: WORK_DURATION,
            pomodoros_completed: 0,
        }
    }

    fn get_pomodoros_progress(&self) -> String {
        let mut progress = String::new();
        // switch logic to cycles_completed

        if self.current_mode == PomodoroMode::LongBreak {
            progress.push('●');
            progress.push(' ');
            progress.push('●');

            progress.push(' ');
            progress.push('●');

            progress.push(' ');
            progress.push('●');

            progress.push(' ');
            return progress;
        }

        let filled = if self.cycles_completed > 0 && self.cycles_completed % 4 == 0 {
            4
        } else {
            self.cycles_completed % 4
        };

        for i in 0..4 {
            if i < filled {
                progress.push('●');

                progress.push(' ');
            } else {
                progress.push('○');

                progress.push(' ');
            }
        }
        progress
    }
}

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
            let outer_block = Block::new()
                .title(Line::from("Boomodoro").centered())
                .borders(Borders::ALL);
            let inner_area = outer_block.inner(frame.area());
            frame.render_widget(outer_block, frame.area());

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Min(1),
                    Constraint::Percentage(10),
                    Constraint::Fill(1),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                ])
                .split(inner_area);

            let current_mode = Paragraph::new(pomo.current_mode.to_string()).centered();

            let current_mode_title = BigText::builder()
                .pixel_size(PixelSize::Octant)
                .centered()
                .style(Style::new().bold())
                .lines(vec![pomo.current_mode.to_string().into()])
                .build();

            let time_remaining = BigText::builder()
                .pixel_size(PixelSize::HalfHeight)
                .centered()
                .style(Style::new().bold())
                .lines(vec![format_time(pomo.time_remaining).into()])
                .build();

            let hints = Paragraph::new(format!(
                "(p-{}) (q-quit) (r-reset) (j-jump to next)",
                if pomo.is_paused { "start" } else { "stop" }
            ))
            .centered();

            let pomodoros_completed = Paragraph::new(pomo.get_pomodoros_progress()).centered();

            frame.render_widget(current_mode_title, layout[1]);

            frame.render_widget(pomodoros_completed, layout[3]);

            frame.render_widget(time_remaining, layout[2]);
            frame.render_widget(
                hints.block(
                    Block::new()
                        .title(Line::from("Keybindings").centered())
                        .borders(Borders::ALL),
                ),
                layout[4],
            );
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

fn format_time(time_in_secs: u32) -> String {
    // time_in_secs % 60 will give secs
    // time_in_secs / 60 will give mins
    // pad them to 00:00
    let minutes = format!("{:0>2}", time_in_secs / 60);
    let seconds = format!("{:0>2}", time_in_secs % 60);

    format!("{} : {}", minutes, seconds)
}

fn play_notes(notes: &[(f32, f32)]) {
    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stram");
    let player = rodio::Player::connect_new(&handle.mixer());
    handle.log_on_drop(false);

    for note in notes {
        let sine = SineWave::new(note.0)
            .take_duration(Duration::from_secs_f32(note.1))
            .amplify(0.50);

        player.append(sine);
        player.sleep_until_end();
    }
}
