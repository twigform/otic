slint::include_modules!();

use slint::{ModelRc, SharedString, VecModel};
use std::rc::Rc;

mod dir;

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
    let weak_window = main_window.as_weak();

    let tracks_thing: Rc<VecModel<Track>> = Rc::new(VecModel::from(dummy_fill()));

    main_window
        .global::<PlayerState>()
        .set_tracks(ModelRc::from(tracks_thing.clone()));

    main_window
        .global::<PlayerState>()
        .on_select_track(move |index| {
            let Some(window) = weak_window.upgrade() else {
                return;
            };

            window.global::<PlayerState>().set_selected_index(index);

            println!("clicked track {index}");
        });

    let init_dirs: Vec<SharedString> = dir::list_dirs()
        .unwrap_or_default()
        .into_iter()
        .map(|p| p.display().to_string().into())
        .collect();

    let dirs_thing: Rc<VecModel<SharedString>> = Rc::new(VecModel::from(init_dirs));

    main_window
        .global::<DirsState>()
        .set_directories(ModelRc::from(dirs_thing.clone()));

    main_window
        .global::<DirsState>()
        .on_add_directory(move || match dir::add_dir() {
            Ok(Some(chosen)) => {
                dirs_thing.push(chosen.display().to_string().into());
            }
            Ok(None) => {}
            Err(e) => {
                eprintln!("ow... {e}");
            }
        });

    main_window.run()
}
