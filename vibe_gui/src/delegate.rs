use druid::{commands, AppDelegate, Command, DelegateCtx, Env, Handled, Target};

use crate::data::AppState;

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.set_path(file_info.path().to_str().unwrap());
            data.initialize_player();
            return Handled::Yes;
        }
        Handled::No
    }
}
