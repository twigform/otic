use rodio::source::EmptyCallback;
use rodio::{Decoder, MixerDeviceSink, Source};
use std::fs::File;
use std::time::Duration;

pub struct Player {
    pub device: MixerDeviceSink,
    pub current: Option<rodio::Player>,
    pub volume: f32,
    pub duration: Option<Duration>,
}

pub fn new_player() -> Player {
    let device = rodio::DeviceSinkBuilder::open_default_sink().expect("no audio output device?!");
    Player {
        device,
        current: None,
        volume: 0.5,
        duration: None,
    }
}

pub fn pos(player: &Player) -> Duration {
    player
        .current
        .as_ref()
        .map(|p| p.get_pos())
        .unwrap_or_default()
}

pub fn seek(player: &Player, to: Duration) {
    if let Some(current) = &player.current {
        if let Err(e) = current.try_seek(to) {
            eprintln!("seek error: {e:?}");
        }
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

    player.duration = source.total_duration();
    player.current = None;
    let p = rodio::Player::connect_new(player.device.mixer());
    p.set_volume(player.volume);
    p.append(source);
    p.append(EmptyCallback::new(Box::new(on_finished)));
    player.current = Some(p);
}
