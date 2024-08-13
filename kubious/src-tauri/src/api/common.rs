pub mod kubious_api {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use tauri::AppHandle;

    use crate::api::{
        application_api::ApplicationCommand, artifacts_api::ArtifactsCommand,
        helm_api::HelmCommand, kompose_api::KomposeCommand, kube_api::KubeCommand,
    };

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(tag = "scope")]
    pub enum ApiCommand {
        Application(ApplicationCommand),
        Kube(KubeCommand),
        Helm(HelmCommand),
        Kompose(KomposeCommand),
        Artifacts(ArtifactsCommand),
    }

    pub trait CommandHandler {
        fn wrap_in_value(&self, result: Result<impl Serialize, String>) -> Result<Value, String> {
            match result {
                Ok(success) => Ok(serde_json::to_value(success).unwrap()),
                Err(error) => Err(error)
            }
        }

        async fn execute(&self, _handle: &AppHandle) -> Result<Value, String> {
            Err::<Value, String>("Execution not implemented".into())
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct CommandResult {
        command: ApiCommand,
        success: bool,
        value: Option<Value>,
        error: Option<String>,
    }

    fn unwrap_result(command: ApiCommand, result: Result<impl Serialize, String>) -> CommandResult {
        match result {
            Ok(res) => serde_json::to_value(res)
                .and_then(|v| Ok(CommandResult {
                    command: command.clone(),
                    success: true,
                    value: Some(v),
                    error: None,
                })).or_else(|_| Ok::<CommandResult, ()>(CommandResult {
                    command: command.clone(),
                    success: false,
                    value: None,
                    error: Some("Failed to parse return value.".into()),
                })),
            Err(res) => Ok(CommandResult {
                    command,
                    success: false,
                    value: None,
                    error: Some(res),
                }),
        }.unwrap()
    }

    pub async fn execute_command(app: AppHandle, command: ApiCommand) -> CommandResult {
        let result = match command.clone() {
            ApiCommand::Application(cmd) => unwrap_result(command, cmd.execute(&app.clone()).await),
            ApiCommand::Kube(cmd) => unwrap_result(command, cmd.execute(&app.clone()).await),
            ApiCommand::Helm(cmd) => unwrap_result(command, cmd.execute(&app.clone()).await),
            ApiCommand::Kompose(cmd) => unwrap_result(command, cmd.execute(&app.clone()).await),
            ApiCommand::Artifacts(cmd) => unwrap_result(command, cmd.execute(&app.clone()).await),
        };

        result
    }
}
