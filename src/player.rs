use rodio::{Decoder, MixerDeviceSink};
use std::fs::File;

pub struct Player {
    pub device: MixerDeviceSink,
    pub current: Option<rodio::Player>,
}

pub fn new_player() -> Player {
    let device = rodio::DeviceSinkBuilder::open_default_sink().expect("no audio output device?!");
    Player {
        device,
        current: None,
    }
}

pub fn play_file(player: &mut Player, path: &str) {
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
    p.append(source);
    player.current = Some(p);
}
