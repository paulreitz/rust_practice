#![windows_subsystem = "windows"]

use druid::{WindowDesc, AppLauncher, Point};
use crate::ui::window::build_ui;
use crate::data::window_data::get_window_data;

mod ui;
pub mod events;
pub mod data;

fn main() {
    let window_data = get_window_data();
    let window_position = Point::new(window_data.window_position.x, window_data.window_position.y);
    let window_size = window_data.window_size;

    let window = WindowDesc::new(build_ui())
        .title("Mellow World")
        .set_position(window_position)
        .window_size((window_size.width, window_size.height));
    AppLauncher::with_window(window)
        // .log_to_console()
        .launch(0u32)
        .expect("Failed to launch application");
}
