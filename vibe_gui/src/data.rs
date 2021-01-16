use std::{fs::File, path::Path};

use druid::{Data, Env, EventCtx, Lens};
use vibe_core::decoder::Decoder;
use vibe_engine::player::Player;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    #[data(ignore)]
    player: Option<Player>,

    play: bool,
    stop: bool,
    filename: String,
    path: String,
    progress: f64,
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
            let file = File::open(path).expect("File not found");
            let decoder = Decoder::new(file).expect("Decoding error");
            let duration = decoder.info().duration().unwrap();

            player.create_stream(decoder);

            let duration = duration.as_millis() as u64;

            Self {
                player: Some(player),
                play: false,
                stop: false,
                filename: filename.into(),
                path: path.into(),
                progress: 0.0,
                duration,
            }
        } else {
            Self {
                player: None,
                play: false,
                stop: false,
                filename: "".into(),
                path: "".into(),
                progress: 0.0,
                duration: 0,
            }
        }
    }

    pub fn get_play(&self) -> bool {
        self.play
    }

    pub fn get_progress(&self) -> f64 {
        self.progress
    }

    fn play_action(&mut self) {
        if self.player.is_some() {
            self.play = !self.play;

            if self.stop {
                self.stop = false;

                let path = self.path.as_str();
                let file = File::open(path).expect("File not found");
                let decoder = Decoder::new(file).expect("Decoding error");
                let mut player = self.player.as_ref().unwrap().clone();

                player.create_stream(decoder);

                self.player = Some(player);
                self.player.as_ref().unwrap().play_stream();
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
        if !self.stop {
            self.stop = true;
            self.play = false;

            if self.player.is_some() {
                self.player.as_ref().unwrap().stop_stream();
            }
        }
    }

    fn select_path(&mut self) {}

    pub fn get_path(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.select_path();
    }

    pub fn toggle_play(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.play_action();
    }

    pub fn toggle_stop(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.stop_action();
    }
}
