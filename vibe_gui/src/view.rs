use druid::{
    widget::{Button, Flex, Label},
    Widget, WidgetExt,
};

use crate::data::*;

fn track() -> impl Widget<AppState> {
    let filename = Label::raw().lens(AppState::filename);

    Flex::row().with_child(filename)
}

fn buttons() -> impl Widget<AppState> {
    let play_button = Button::new("Play").on_click(AppState::toggle_play);
    let stop_button = Button::new("Stop").on_click(AppState::toggle_stop);

    Flex::row().with_child(play_button).with_child(stop_button)
}

pub fn build_ui() -> impl Widget<AppState> {
    Flex::row().with_child(buttons()).with_child(track())
}
