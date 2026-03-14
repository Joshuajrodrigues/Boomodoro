use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::{timer::PomodoroTimer, utils::format_time};

pub fn draw_frame(frame: &mut Frame, pomo: &PomodoroTimer) {
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
}
