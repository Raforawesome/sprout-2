use std::{
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use serde::{Deserialize, Serialize};

/*
 * Global app constants that may be modified at runtime (therefore static)
 */
pub static CONFIG_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| dirs::config_dir().unwrap().join("sprout"));

pub static SETTINGS: LazyLock<Mutex<Settings>> = LazyLock::new(|| {
    let path = CONFIG_DIR.join("settings.json");
    let settings = if path.exists() {
        serde_json::from_str(&std::fs::read_to_string(path).expect("valid settings path"))
            .expect("valid settings file")
    } else {
        Settings::default()
    };
    Mutex::new(settings)
});

/*
 * Holds app settings
 */
#[derive(Serialize, Deserialize, Default)]
pub struct Settings {
    game_path: Option<String>,
}

impl Settings {
    pub fn game_path(&self) -> Option<&str> {
        self.game_path.as_deref()
    }

    pub fn set_game_path(&mut self, game_path: Option<String>) {
        self.game_path = game_path;
        self.save();
    }

    fn save(&self) {
        let path = CONFIG_DIR.join("settings.json");
        std::fs::create_dir_all(&*CONFIG_DIR).expect("failed to create config directory");
        std::fs::write(
            path,
            serde_json::to_string_pretty(self).expect("failed to serialize settings"),
        )
        .expect("failed to write settings file");
    }
}

pub fn get_settings() -> std::sync::MutexGuard<'static, Settings> {
    SETTINGS.lock().expect("failed to grab settings")
}
