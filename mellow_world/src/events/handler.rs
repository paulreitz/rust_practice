use std::fs;

use directories::ProjectDirs;
use druid::{Widget, Size, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, PaintCtx, Point};

use crate::data::{APP_DOMAIN, AUTHOR, APP_NAME, SETTINGS_FILE, window_data::WindowData};

pub struct WindowCloseHandler;

impl Widget<u32> for WindowCloseHandler {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut u32, _env: &Env) {
        match event {
            Event::WindowCloseRequested => {
                let win = ctx.window();
                let Point{x: x_pos, y: y_pos} = win.get_position();
                let Size{width, height} = win.get_size();
                let window_data = WindowData::new_from(x_pos, y_pos, width, height);
                if let Some(project_dir) = ProjectDirs::from(APP_DOMAIN, AUTHOR, APP_NAME) {
                    let config_string = serde_json::to_string(&window_data).unwrap();
                    let config_path = project_dir.data_dir().join(SETTINGS_FILE);
                    fs::create_dir_all(project_dir.data_dir()).expect("Unable to create settings directory");
                    fs::write(config_path, config_string).expect("Unable to write settings file");
                }
            },
            _ => (),
        }
    }
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &u32, _env: &Env) {}
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &u32, _data: &u32, _env: &Env) {}
    fn layout(&mut self, _ctx: &mut LayoutCtx, _bc: &BoxConstraints, _data: &u32, _env: &Env) -> Size {
        Size::ZERO
    }
    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &u32, _env: &Env) {}
}