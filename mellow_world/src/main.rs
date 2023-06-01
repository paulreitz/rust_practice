#![windows_subsystem = "windows"]

use druid::{WindowDesc, AppLauncher};
use crate::ui::window::build_ui;

mod ui;
pub mod events;
pub mod data;

fn main() {
    let window = WindowDesc::new(build_ui())
        .title("Mellow World")
        .window_size((800.0, 600.0));
    AppLauncher::with_window(window)
        .launch(0u32)
        .expect("Failed to launch application");
}
