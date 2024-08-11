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
        fn execute<T: Serialize>(&self, _handle: &AppHandle) -> Result<Box<T>, String> {
            Err("Execution not implemented".into())
        }
    }

    pub fn execute_command<T: Serialize>(app: AppHandle, command: ApiCommand) -> Result<Value, String> {
        let result: Result<Box<T>, String> = match command {
            ApiCommand::Application(cmd) => cmd.execute(&app.clone()),
            ApiCommand::Kube(cmd) => cmd.execute(&app.clone()),
            ApiCommand::Helm(cmd) => cmd.execute(&app.clone()),
            ApiCommand::Kompose(cmd) => cmd.execute(&app.clone()),
            ApiCommand::Artifacts(cmd) => cmd.execute(&app.clone()),
        };
        
        match result {
            Ok(res) => serde_json::to_value(*res).and_then(|v| Ok(v)).or(Err("Failed to parse return value.".into())),
            Err(res) => Err(res)
        }
    }
}