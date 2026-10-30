use crate::Track;
use directories::BaseDirs;
use lofty::prelude::*;
use rusqlite::{Connection, params};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

const AUDIO_EXTS: &[&str] = &["mp3", "flac", "wav", "ogg", "m4a"];

fn is_audio(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| AUDIO_EXTS.contains(&ext))
        .unwrap_or(false)
}

struct Metadata {
    title: String,
    artist: String,
    dur_s: u32,
}

struct Cached {
    mtime: i64,
    size: i64,
    meta: Metadata,
}

fn db_path() -> PathBuf {
    let dir = BaseDirs::new()
        .map(|b| b.config_dir().join("otic"))
        .unwrap_or_else(|| PathBuf::from("."));
    let _ = std::fs::create_dir_all(&dir);
    dir.join("library.db")
}

fn read_mdata(path: &Path) -> Metadata {
    let fallback_title = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "borked title".to_string());

    let Ok(tagged) = lofty::read_from_path(path) else {
        return Metadata {
            title: fallback_title,
            artist: "unknown artist".into(),
            dur_s: 0,
        };
    };

    let dur_s = tagged.properties().duration().as_secs() as u32;

    let tag = tagged.primary_tag().or_else(|| tagged.first_tag());

    let title = tag
        .and_then(|t| t.title())
        .map(|s| s.to_string())
        .unwrap_or(fallback_title);

    let artist = tag
        .and_then(|t| t.artist())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown artist".to_string());

    Metadata {
        title,
        artist,
        dur_s,
    }
}

fn file_check_info(path: &Path) -> Option<(i64, i64)> {
    let md = std::fs::metadata(path).ok()?;
    let mtime = md
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs() as i64;
    Some((mtime, md.len() as i64))
}

fn get_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            get_files(&path, out);
        } else if is_audio(&path) {
            out.push(path);
        }
    }
}

fn to_track(path: &str, meta: &Metadata) -> Track {
    Track {
        title: meta.title.clone().into(),
        artist: meta.artist.clone().into(),
        dur: format!("{}:{:02}", meta.dur_s / 60, meta.dur_s % 60).into(),
        path: path.into(),
    }
}

pub struct Library {
    conn: Connection,
}

impl Library {
    pub fn open() -> rusqlite::Result<Self> {
        let conn = Connection::open(db_path())?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tracks (
                path   TEXT PRIMARY KEY,
                mtime  INTEGER NOT NULL,
                size   INTEGER NOT NULL,
                title  TEXT NOT NULL,
                artist TEXT NOT NULL,
                dur_s  INTEGER NOT NULL
            );",
        )?;
        Ok(Self { conn })
    }

    fn load_cache(&self) -> rusqlite::Result<HashMap<String, Cached>> {
        let mut stmt = self
            .conn
            .prepare("SELECT path, mtime, size, title, artist, dur_s FROM tracks")?;

        let rows = stmt.query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                Cached {
                    mtime: r.get(1)?,
                    size: r.get(2)?,
                    meta: Metadata {
                        title: r.get(3)?,
                        artist: r.get(4)?,
                        dur_s: r.get(5)?,
                    },
                },
            ))
        })?;

        rows.collect()
    }

    pub fn scan_all(&mut self, dirs: &[PathBuf]) -> Vec<Track> {
        match self.try_scan(dirs) {
            Ok(tracks) => tracks,
            Err(e) => {
                eprintln!("library scan failed {e}");
                Vec::new()
            }
        }
    }

    fn try_scan(&mut self, dirs: &[PathBuf]) -> rusqlite::Result<Vec<Track>> {
        let mut cache = self.load_cache()?;

        let mut files = Vec::new();
        for dir in dirs {
            get_files(dir, &mut files);
        }

        let tx = self.conn.transaction()?;
        let mut tracks = Vec::with_capacity(files.len());
        let mut reread = 0;
        {
            let mut upsert = tx.prepare(
                "INSERT OR REPLACE INTO tracks (path, mtime, size, title, artist, dur_s)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )?;

            for path in &files {
                let Some((mtime, size)) = file_check_info(path) else {
                    continue;
                };

                let key = path.display().to_string();

                let metadata = match cache.remove(&key) {
                    Some(c) if c.mtime == mtime && c.size == size => c.meta,
                    _ => {
                        let m = read_mdata(path);
                        upsert.execute(params![key, mtime, size, m.title, m.artist, m.dur_s])?;
                        reread += 1;
                        m
                    }
                };
                tracks.push(to_track(&key, &metadata));
            }
        }

        let removed = cache.len();
        for stale in cache.keys() {
            tx.execute("DELETE FROM tracks WHERE path = ?1", params![stale])?;
        }
        tx.commit()?;

        eprintln!(
            "{} tracks\n{} read from disk\n{} removed",
            tracks.len(),
            reread,
            removed
        );
        Ok(tracks)
    }
}
