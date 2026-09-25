use crate::Track;
use std::path::{Path, PathBuf};

const AUDIO_EXTS: &[&str] = &["mp3", "flac", "wav", "ogg", "m4a"];

fn is_audio(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| AUDIO_EXTS.contains(&ext))
        .unwrap_or(false)
}

fn get_track(path: &Path) -> Track {
    let title = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "borked title".to_string());

    Track {
        title: title.into(),
        artist: "DJ Lorem".into(),
        dur: "3:00".into(),
        path: path.display().to_string().into(),
    }
}

fn scan(dir: &Path, tracks: &mut Vec<Track>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan(&path, tracks);
        } else if is_audio(&path) {
            tracks.push(get_track(&path));
        }
    }
}

pub fn scan_all(dirs: &[PathBuf]) -> Vec<Track> {
    let mut tracks = Vec::new();
    for dir in dirs {
        scan(dir, &mut tracks);
    }
    tracks
}
