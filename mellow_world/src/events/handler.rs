use directories::ProjectDirs;
use druid::{Widget, Size, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, PaintCtx, Point};

pub struct WindowCloseHandler;

impl Widget<u32> for WindowCloseHandler {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut u32, _env: &Env) {
        match event {
            Event::WindowCloseRequested => {
                let win = ctx.window();
                let Point{x: x_pos, y: y_pos} = win.get_position();
                let Size{width, height} = win.get_size();
                println!("WindowClosedRequested: x_pos: {}, y_pos: {}, width: {}, height: {}", x_pos, y_pos, width, height);
                if let Some(project_dir) = ProjectDirs::from("io.paulreitz", "paulreitz", "mellow_world") {
                    println!("ProjectDirs: {:?}", project_dir);
                    // Save current window position and size to app data file.
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