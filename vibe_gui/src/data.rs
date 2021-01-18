use std::{fs::File, path::Path};

use druid::{Command, Data, Env, EventCtx, FileDialogOptions, FileSpec, Lens, Target};
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
    pub fn new(player: Player) -> Self {
        Self {
            player: Some(player),
            play: false,
            stop: false,
            filename: "".into(),
            path: "".into(),
            progress: 0.0,
            duration: 0,
        }
    }

    pub fn initialize_player(&mut self) {
        let path = self.path.as_str();
        let file = File::open(path).expect("File not found");

        self.stop = true;
        self.play = false;
        self.set_filename();

        let decoder = Decoder::new(file).expect("Decoding error");

        let mut player = self.player.as_ref().unwrap().clone();

        player.create_stream(decoder);

        self.player = Some(player);
    }

    pub fn get_play(&self) -> bool {
        self.play
    }

    pub fn get_progress(&self) -> f64 {
        self.progress
    }

    pub fn set_filename(&mut self) {
        let path = self.path.as_str();
        let path = Path::new(path);
        let filename = path.file_name().and_then(|f| f.to_str()).unwrap();

        self.filename = String::from(filename);
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = String::from(path);
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

    pub fn select_path(ctx: &mut EventCtx, _data: &mut Self, _env: &Env) {
        let mp3 = FileSpec::new("MP3 file", &["mp3"]);
        let wav = FileSpec::new("WAV file", &["wav"]);
        let ogg = FileSpec::new("OGG file", &["ogg"]);
        let flac = FileSpec::new("FLAC file", &["flac"]);

        let open_dialog_options = FileDialogOptions::new()
            .allowed_types(vec![mp3, wav, ogg, flac])
            .name_label("Source")
            .title("Choose a file")
            .button_text("Playback");

        ctx.submit_command(Command::new(
            druid::commands::SHOW_OPEN_PANEL,
            open_dialog_options.clone(),
            Target::Auto,
        ))
    }

    pub fn toggle_play(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.play_action();
    }

    pub fn toggle_stop(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.stop_action();
    }
}
