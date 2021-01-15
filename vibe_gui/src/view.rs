use druid::{
    widget::{Button, Flex, Label},
    Widget, WidgetExt,
};

use crate::data::*;

fn track() -> impl Widget<AppState> {
    let filename = Label::raw().lens(AppState::filename);

    Flex::row().with_child(filename)
}

fn get_play_unicode(play: bool) -> String {
    if play {
        return String::from("\u{23f8}");
    } else {
        return String::from("\u{25b6}");
    }
}

fn buttons() -> impl Widget<AppState> {
    let plus_button = Button::new("\u{2795}");
    let play_pause_button: Button<AppState> =
        Button::dynamic(|data: &AppState, _| format!("{}", get_play_unicode(data.get_play())));
    let stop_button = Button::new("\u{23f9}");

    let plus_controller = plus_button.on_click(AppState::get_path);
    let play_pause_controller = play_pause_button.on_click(AppState::toggle_play);
    let stop_controller = stop_button.on_click(AppState::toggle_stop);

    Flex::row()
        .with_spacer(5.0)
        .with_child(plus_controller)
        .with_child(play_pause_controller)
        .with_child(stop_controller)
}

pub fn build_ui() -> impl Widget<AppState> {
    Flex::row()
        .with_child(buttons())
        .with_spacer(20.0)
        .with_child(track())
}
