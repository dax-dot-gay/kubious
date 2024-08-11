pub mod kubious_api {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use tauri::AppHandle;

    use crate::api::{application_api::ApplicationCommand, artifacts_api::ArtifactsCommand, helm_api::HelmCommand, kompose_api::KomposeCommand, kube_api::KubeCommand};


    #[derive(Serialize, Deserialize, Clone)]
    pub enum ApiCommand {
        Application(ApplicationCommand),
        Kube(KubeCommand),
        Helm(HelmCommand),
        Kompose(KomposeCommand),
        Artifacts(ArtifactsCommand)
    }

    pub trait CommandHandler {
        fn execute(&self, _handle: &AppHandle) -> Result<impl Serialize, String> {
            Err::<Value, String>("Execution not implemented".into())
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct CommandResult {
        command: ApiCommand,
        success: bool,
        value: Option<Value>,
        error: Option<String>
    }

    pub fn execute_command(app: AppHandle, command: ApiCommand) -> CommandResult {
        let result = match command.clone() {
            ApiCommand::Application(cmd) => serde_json::to_value(cmd.execute(&app.clone())),
            ApiCommand::Kube(cmd) => serde_json::to_value(cmd.execute(&app.clone())),
            ApiCommand::Helm(cmd) => serde_json::to_value(cmd.execute(&app.clone())),
            ApiCommand::Kompose(cmd) => serde_json::to_value(cmd.execute(&app.clone())),
            ApiCommand::Artifacts(cmd) => serde_json::to_value(cmd.execute(&app.clone())),
        };
        
        match result {
            Ok(res) => CommandResult{command, success: true, value: Some(res), error: None},
            Err(_) => CommandResult{command, success: false, value: None, error: Some("Failed to parse return value.".into())}
        }
    }
}