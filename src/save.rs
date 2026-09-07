use crate::mission::Mission;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Progress {
    pub version: u32,
    pub completed: BTreeMap<String, usize>,
    pub volume: f32,
    pub show_grid: bool,
    pub slow: bool,
}
impl Default for Progress {
    fn default() -> Self {
        Self {
            version: 1,
            completed: BTreeMap::new(),
            volume: 0.35,
            show_grid: false,
            slow: false,
        }
    }
}
impl Progress {
    pub fn unlocked(&self, missions: &[Mission], index: usize) -> bool {
        index < missions.len()
            && (index == 0
                || self.completed.contains_key(missions[index - 1].id)
                || self.completed.contains_key(missions[index].id))
    }
    pub fn complete(&mut self, id: &str, cards: usize) {
        self.completed
            .entry(id.into())
            .and_modify(|best| *best = (*best).min(cards))
            .or_insert(cards);
    }
    pub fn load(path: &Path) -> (Self, Option<String>) {
        match fs::read(path) {
            Ok(bytes) => match serde_json::from_slice::<Self>(&bytes) {
                Ok(mut p) if p.version == 1 => { p.volume = p.volume.clamp(0.,1.); (p, None) }
                _ => (Self::default(), Some("The save could not be read. Your original file is kept; this session won't overwrite it.".into())),
            },
            Err(e) if e.kind() == io::ErrorKind::NotFound => (Self::default(), None),
            Err(e) => (Self::default(), Some(format!("Couldn't read progress: {e}"))),
        }
    }
    pub fn write(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let bytes = serde_json::to_vec_pretty(self)?;
        let temp = path.with_extension("tmp");
        {
            use std::io::Write;
            let mut f = fs::File::create(&temp)?;
            f.write_all(&bytes)?;
            f.sync_all()?;
        }
        fs::rename(temp, path)
    }
}
pub fn default_path() -> PathBuf {
    if let Some(dir) = std::env::var_os("BARKELY_DATA_DIR") {
        return PathBuf::from(dir).join("progress.json");
    }
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    #[cfg(target_os = "macos")]
    let root = home.join("Library/Application Support");
    #[cfg(target_os = "windows")]
    let root = std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .unwrap_or(home);
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let root = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local/share"));
    root.join("BarkelyHighlands").join("progress.json")
}
