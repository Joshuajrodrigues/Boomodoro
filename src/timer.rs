use std::{fmt, time::Instant};

use notify_rust::{Notification, Timeout};

use crate::audio::{play_notes, NOTES_PAUSE, NOTES_RESET, NOTES_SKIP, NOTES_TRANSITION};

#[derive(Debug, PartialEq)]
pub enum PomodoroMode {
    Work,
    ShortBreak,
    LongBreak,
}

const LONG_DURATION: u32 = 15 * 60;
const SHORT_DURATION: u32 = 5 * 60;
const WORK_DURATION: u32 = 25 * 60;

impl fmt::Display for PomodoroMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PomodoroMode::Work => write!(f, "Work"),
            PomodoroMode::ShortBreak => write!(f, "Short Break"),
            PomodoroMode::LongBreak => write!(f, "Long Break :)"),
        }
    }
}

pub struct PomodoroTimer {
    pub is_paused: bool,
    pub current_mode: PomodoroMode,
    pub time_remaining: u32,
    pub pomodoros_completed: u32,
    pub time_running: Instant,
    pub cycles_completed: u32,
}

impl PomodoroTimer {
    pub fn tick(&mut self) {
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
                std::thread::spawn(|| play_notes(NOTES_TRANSITION));
            }
        }
    }

    pub fn toggle_paused(&mut self) {
        std::thread::spawn(|| play_notes(NOTES_PAUSE));
        self.is_paused = !self.is_paused;
    }

    pub fn get_next_mode(&self) -> PomodoroMode {
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

    pub fn transition(&mut self, stage: PomodoroMode) {
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

    pub fn reset(&mut self) {
        std::thread::spawn(|| play_notes(NOTES_RESET));
        self.is_paused = true;
        self.cycles_completed = 0;
        self.current_mode = PomodoroMode::Work;
        self.pomodoros_completed = 0;
        self.time_running = Instant::now();
        self.time_remaining = WORK_DURATION;
    }

    pub fn skip(&mut self) {
        if self.current_mode == PomodoroMode::Work {
            self.cycles_completed += 1;
            self.pomodoros_completed += 1;
        }
        let next_mode = self.get_next_mode();
        self.transition(next_mode);

        std::thread::spawn(|| play_notes(NOTES_SKIP));
    }

    pub fn new() -> PomodoroTimer {
        PomodoroTimer {
            time_running: Instant::now(),
            is_paused: true,
            cycles_completed: 0,
            current_mode: PomodoroMode::Work,
            time_remaining: WORK_DURATION,
            pomodoros_completed: 0,
        }
    }

    pub fn get_pomodoros_progress(&self) -> String {
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
