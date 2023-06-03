use std::fs;

use druid::{Data, Lens};
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;

use crate::data::{APP_DOMAIN, AUTHOR, APP_NAME, SETTINGS_FILE};

#[derive(Debug, Clone, Data, Lens, Serialize, Deserialize, Default)]
pub struct WindowData {
    pub window_position: WindowPosition,
    pub window_size: WindowSize,
}

impl WindowData {
    pub fn new_from(x: f64, y: f64, width: f64, height: f64) -> Self {
        WindowData {
            window_position: WindowPosition {
                x: x,
                y: y,
            },
            window_size: WindowSize {
                width: width,
                height: height,
            },
        }
    }
}

#[derive(Debug, Clone, Data, Lens, Serialize, Deserialize, Default)]
pub struct WindowPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Data, Lens, Serialize, Deserialize, Default)]
pub struct WindowSize {
    pub width: f64,
    pub height: f64,
}

pub fn get_window_data() -> WindowData {
    if let Some(project_dir) = ProjectDirs::from(APP_DOMAIN, AUTHOR, APP_NAME) {
        let config_path = project_dir.data_dir().join(SETTINGS_FILE);
        match fs::read_to_string(config_path) {
            Ok(data) => {
                let window_data: WindowData = serde_json::from_str(&data).unwrap();
                return window_data
            },
            Err(_) => {
                return WindowData::new_from(100.0, 100.0, 800.0, 600.0)
            },
        };
    }
    WindowData::new_from(100.0, 100.0, 800.0, 600.0)
}