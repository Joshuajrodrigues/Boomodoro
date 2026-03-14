use rodio::source::SineWave;
use rodio::Source;
use std::time::Duration;

pub const NOTES_TRANSITION: &[(f32, f32)] = &[(261.0, 0.25), (330.0, 0.25), (261.0, 0.25)];
pub const NOTES_SKIP: &[(f32, f32)] = &[(261.0, 0.25)];
pub const NOTES_PAUSE: &[(f32, f32)] = &[(330.0, 0.25)];
pub const NOTES_QUIT: &[(f32, f32)] = &[(440.0, 0.25)];
pub const NOTES_RESET: &[(f32, f32)] = &[(330.0, 0.25), (261.0, 0.25), (330.0, 0.25)];

pub fn play_notes(notes: &[(f32, f32)]) {
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
