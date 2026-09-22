slint::include_modules!();

use slint::{ModelRc, VecModel};
use std::rc::Rc;

fn dummy_fill() -> Vec<Track> {
    vec![
        Track {
            title: "dummy song 1".into(),
            artist: "dummy artist".into(),
            dur: "3:01".into(),
        },
        Track {
            title: "dummy song 2".into(),
            artist: "dummy artist".into(),
            dur: "3:02".into(),
        },
        Track {
            title: "dummy song 3".into(),
            artist: "dummy artist".into(),
            dur: "3:03".into(),
        },
        Track {
            title: "Eine kleine Nachtmusik (K. 525)".into(),
            artist: "Wolfgang Amadeus Mozart".into(),
            dur: "18:06".into(),
        },
    ]
}

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    let tracks_thing: Rc<VecModel<Track>> = Rc::new(VecModel::from(dummy_fill()));

    main_window
        .global::<PlayerState>()
        .set_tracks(ModelRc::from(tracks_thing.clone()));

    main_window.run()
}
