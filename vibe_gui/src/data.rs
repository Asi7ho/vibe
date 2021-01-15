use std::path::Path;

use druid::{Data, Env, EventCtx, Lens};
use vibe_engine::player::Player;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    #[data(ignore)]
    player: Option<Player>,
    play: bool,
    stop: bool,
    filename: String,
    path: String,
    duration: u64,
}

impl AppState {
    pub fn new<P: AsRef<Path>>(mut player: Player, path: P) -> Self {
        if path.as_ref().is_file() {
            let filename = path
                .as_ref()
                .file_name()
                .and_then(|filename| filename.to_str())
                .unwrap();

            let path = path.as_ref().to_str().unwrap();
            player.play_audio(path);

            Self {
                player: Some(player),
                play: false,
                stop: false,
                filename: filename.into(),
                path: path.into(),
                duration: 0,
            }
        } else {
            Self {
                player: None,
                play: false,
                stop: false,
                filename: "".into(),
                path: "".into(),
                duration: 0,
            }
        }
    }

    fn play_action(&mut self) {
        self.play = !self.play;

        if self.player.is_some() {
            if self.stop {
                self.stop = false;

                let path = self.path.as_str();
                let mut player = self.player.as_ref().unwrap().clone();

                player.play_audio(path);
            } else {
                if self.play {
                    self.player.as_ref().unwrap().play_stream();
                } else {
                    self.player.as_ref().unwrap().pause_stream();
                }
            }
        }
    }

    fn stop_action(&mut self) {
        self.stop = true;

        if self.player.is_some() {
            self.player.as_ref().unwrap().stop_stream();
        }
    }

    pub fn toggle_play(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.play_action();
    }

    pub fn toggle_stop(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.stop_action();
    }
}
