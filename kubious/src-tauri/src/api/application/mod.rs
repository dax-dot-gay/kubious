pub mod application_api {
    use std::{fs::File, io::Write};

    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use tauri::Manager;
    use crate::CommandHandler;

    use super::app_state::AppState;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(tag = "command")]
    pub enum ApplicationCommand {
        SetCurrentConfig{key: Option<String>},
        GetCurrentConfig{},
        GetConfigs{}
    }
    impl CommandHandler for ApplicationCommand {
        fn execute(&self, handle: &tauri::AppHandle) -> Result<Value, String> {
            match self {
                ApplicationCommand::SetCurrentConfig { key } => {
                    let state = handle.state::<AppState>();
                    let new_conf = state.set_current_config(key.clone());
                    if let Ok(conf) = new_conf {
                        if let Ok(path) = handle.path().parse("$APPCONFIG/config.json") {
                            let mut config_file = File::create(path).unwrap();
                            let jsonified = state.to_json().unwrap();
                            config_file.write_all(jsonified.as_bytes()).unwrap();
                            return self.wrap_in_value(Ok(conf));
                        } else {
                            return Err("Failed to write new current config to file.".to_string());
                        }
                    } else {
                        return Err("Unknown config key".to_string())
                    }
                },
                ApplicationCommand::GetConfigs {} => self.wrap_in_value(Ok(handle.state::<AppState>().get_configs())),
                ApplicationCommand::GetCurrentConfig {} => self.wrap_in_value(Ok(handle.state::<AppState>().get_current_config()))
            }
        }
    }
}

mod state;
pub use state::app_state;