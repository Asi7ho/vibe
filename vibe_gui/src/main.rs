use druid::{AppLauncher, WindowDesc};

mod data;
use data::AppState;

mod view;
use vibe_engine::player::Player;
use view::build_ui;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Vibe Player")
        .window_size((400.0, 100.0));

    let player = Player::new().unwrap();
    let initial_state = AppState::new(
        player,
        "/Users/yanndebain/Documents/GitHub/vibe/vibe_core/tests/sounds/Test1.mp3",
    );

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
