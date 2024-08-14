pub mod application_api {

    use std::collections::HashMap;

    use crate::{compat::kube_compat::KubeConfig, CommandHandler};
    use k8s_openapi::apimachinery::pkg::version::Info;
    use kube::config::Kubeconfig;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use tauri::Manager;

    use super::app_state::AppState;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ConfigCheck {
        config: KubeConfig,
        connected: bool,
        version: Option<Info>
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(tag = "command")]
    pub enum ApplicationCommand {
        SetCurrentConfig { key: Option<String> },
        GetCurrentConfig {},
        GetConfigs {},
        AddConfig { key: String, config: Kubeconfig },
        RemoveConfig { key: String },
        CheckConfigs {},
        CheckConfig {key: String}
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
                },
                ApplicationCommand::CheckConfig { key } => {
                    let state = handle.state::<AppState>();
                    if let Some(config) = state.select_config(key) {
                        if let Some(client) = state.client_for(key).await {
                            if let Ok(vers) = client.apiserver_version().await {
                                self.wrap_in_value(Ok(ConfigCheck {config, connected: true, version: Some(vers)}))
                            } else {
                                self.wrap_in_value(Ok(ConfigCheck {config, connected: false, version: None}))
                            }
                        } else {
                            self.wrap_in_value(Ok(ConfigCheck {config, connected: false, version: None}))
                        }
                    } else {
                        Err("Unknown config key".to_string())
                    }
                },
                ApplicationCommand::CheckConfigs {  } => {
                    let state = handle.state::<AppState>();
                    let mut config_mapping: HashMap<String, ConfigCheck> = HashMap::new();
                    for (key, config) in state.get_configs() {
                        if let Some(client) = state.client_for(key.as_str()).await {
                            if let Ok(vers) = client.apiserver_version().await {
                                config_mapping.insert(key, ConfigCheck {config, connected: true, version: Some(vers)});
                            } else {
                                config_mapping.insert(key, ConfigCheck {config, connected: false, version: None});
                            }
                        } else {
                            config_mapping.insert(key, ConfigCheck {config, connected: false, version: None});
                        }
                    }
                    self.wrap_in_value(Ok(config_mapping))
                }
            }
        }
    }
}

mod state;
pub use state::app_state;
