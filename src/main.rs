slint::include_modules!();

use slint::{Model, ModelRc, SharedString, VecModel};
use std::cell::RefCell;
use std::rc::Rc;

mod dir;
mod library;
mod player;

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    let weak_window = main_window.as_weak();

    let player = Rc::new(RefCell::new(player::new_player()));

    // track stuff

    let init_dirs = dir::list_dirs().unwrap_or_default();
    let initial_tracks = library::scan_all(&init_dirs);

    let tracks_thing: Rc<VecModel<Track>> = Rc::new(VecModel::from(initial_tracks));

    main_window
        .global::<PlayerState>()
        .set_tracks(ModelRc::from(tracks_thing.clone()));

    main_window.global::<PlayerState>().on_select_track({
        let weak_window = weak_window.clone();
        let tracks_thing = tracks_thing.clone();
        let player = player.clone();

        move |index| {
            let Some(window) = weak_window.upgrade() else {
                return;
            };

            window.global::<PlayerState>().set_selected_index(index);

            if let Some(track) = tracks_thing.row_data(index as usize) {
                player::play_file(&mut player.borrow_mut(), &track.path);
            }

            println!("clicked track {index}");
        }
    });

    // directory stuff

    let init_dirs_vec: Vec<SharedString> = init_dirs
        .iter()
        .map(|p| p.display().to_string().into())
        .collect();

    let dirs_thing: Rc<VecModel<SharedString>> = Rc::new(VecModel::from(init_dirs_vec));

    main_window
        .global::<DirsState>()
        .set_directories(ModelRc::from(dirs_thing.clone()));

    main_window.global::<DirsState>().on_add_directory({
        let tracks_thing = tracks_thing.clone();
        let dirs_thing = dirs_thing.clone();

        move || match dir::add_dir() {
            Ok(Some(chosen)) => {
                let new_tracks = library::scan_all(&[chosen.clone()]);
                for track in new_tracks {
                    tracks_thing.push(track);
                }

                dirs_thing.push(chosen.display().to_string().into());
            }
            Ok(None) => {}
            Err(e) => {
                eprintln!("wasn't able to add dir: {e}");
            }
        }
    });

    main_window.global::<DirsState>().on_rm_directory({
        let tracks_thing = tracks_thing.clone();
        let dirs_thing = dirs_thing.clone();

        move |index| {
            let index = index as usize;

            match dir::rm_dir(index) {
                Ok(()) => {
                    dirs_thing.remove(index);

                    let all_dirs = dir::list_dirs().unwrap_or_default();
                    let new_tracks = library::scan_all(&all_dirs);
                    tracks_thing.set_vec(new_tracks);
                }
                Err(e) => {
                    eprintln!("wasn't able to remove dir: {e}");
                }
            }
        }
    });

    // window controls

    let window_state = main_window.global::<WindowState>();

    {
        let weak_window = weak_window.clone();
        window_state.on_minimize(move || {
            if let Some(window) = weak_window.upgrade() {
                window.window().set_minimized(true);
            }
        });
    }

    {
        let weak_window = weak_window.clone();
        window_state.on_toggle_fullscreen(move || {
            if let Some(window) = weak_window.upgrade() {
                let win = window.window();
                let is_fs = win.is_maximized();
                win.set_maximized(!is_fs);
            }
        });
    }

    {
        let weak_window = weak_window.clone();
        window_state.on_close(move || {
            if let Some(window) = weak_window.upgrade() {
                let _ = window.hide();
            }
        });
    }

    main_window.run()
}
