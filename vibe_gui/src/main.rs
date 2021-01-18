use druid::{AppLauncher, WindowDesc};

mod data;
use data::AppState;

mod view;
use vibe_engine::player::Player;
use view::build_ui;

mod delegate;
use delegate::Delegate;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Vibe Player")
        .window_size((400.0, 50.0));

    let player = Player::new().unwrap();
    let initial_state = AppState::new(player);

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}
