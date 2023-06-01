use druid::{Widget, widget::{Flex, Label}};
use crate::events::handler::WindowCloseHandler;

pub fn build_ui() -> impl Widget<u32> {
    Flex::column()
        .with_child(Label::new("Mellow World!"))
        .with_child(WindowCloseHandler)
}