pub mod application_api {

    use crate::CommandHandler;
    use kube::config::Kubeconfig;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use tauri::Manager;

    use super::app_state::AppState;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(tag = "command")]
    pub enum ApplicationCommand {
        SetCurrentConfig { key: Option<String> },
        GetCurrentConfig {},
        GetConfigs {},
        AddConfig { key: String, config: Kubeconfig },
        RemoveConfig { key: String },
    }
    impl CommandHandler for ApplicationCommand {
        async fn execute(&self, handle: &tauri::AppHandle) -> Result<Value, String> {
            match self {
                ApplicationCommand::SetCurrentConfig { key } => {
                    let state = handle.state::<AppState>();
                    let new_conf = state.set_current_config(key.clone());
                    if let Ok(conf) = new_conf {
                        state
                            .save_state(handle.clone())
                            .and(self.wrap_in_value(Ok(conf)))
                            .or(Err("Failed to save state".to_string()))
                    } else {
                        return Err("Unknown config key".to_string());
                    }
                }
                ApplicationCommand::GetConfigs {} => {
                    self.wrap_in_value(Ok(handle.state::<AppState>().get_configs()))
                }
                ApplicationCommand::GetCurrentConfig {} => {
                    self.wrap_in_value(Ok(handle.state::<AppState>().get_current_config()))
                }
                ApplicationCommand::AddConfig { key, config } => {
                    let state = handle.state::<AppState>();
                    let conf = state.put_kubeconfig(key, config.clone()).await;
                    state
                        .save_state(handle.clone())
                        .and(self.wrap_in_value(Ok(conf)))
                        .or(Err("Failed to save state".to_string()))
                }
                ApplicationCommand::RemoveConfig { key } => {
                    let state = handle.state::<AppState>();
                    state.remove_config(key);
                    state
                        .save_state(handle.clone())
                        .and(self.wrap_in_value(Ok(())))
                        .or(Err("Failed to save state".to_string()))
                }
            }
        }
    }
}

mod state;
pub use state::app_state;
