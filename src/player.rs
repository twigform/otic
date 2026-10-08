use rodio::source::EmptyCallback;
use rodio::{Decoder, MixerDeviceSink};
use std::fs::File;

pub struct Player {
    pub device: MixerDeviceSink,
    pub current: Option<rodio::Player>,
    pub volume: f32,
}

pub fn new_player() -> Player {
    let device = rodio::DeviceSinkBuilder::open_default_sink().expect("no audio output device?!");
    Player {
        device,
        current: None,
        volume: 0.5,
    }
}

pub fn set_volume(player: &mut Player, volume: f32) {
    player.volume = volume;
    if let Some(current) = &player.current {
        current.set_volume(player.volume);
    }
}

pub fn toggle(player: &mut Player) {
    if let Some(current) = &player.current {
        if current.is_paused() {
            current.play();
        } else {
            current.pause();
        }
    }
}

pub fn playing(player: &Player) -> bool {
    player
        .current
        .as_ref()
        .map(|p| !p.is_paused())
        .unwrap_or(false)
}

pub fn play_file(player: &mut Player, path: &str, on_finished: impl Fn() + Send + 'static) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("couldn't open file {e}");
            return;
        }
    };

    let source = match Decoder::try_from(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("couldn't decode file {e}");
            return;
        }
    };

    player.current = None;
    let p = rodio::Player::connect_new(player.device.mixer());
    p.set_volume(player.volume);
    p.append(source);
    p.append(EmptyCallback::new(Box::new(on_finished)));
    player.current = Some(p);
}
