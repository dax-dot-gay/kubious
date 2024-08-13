pub mod helm_api {
    use crate::CommandHandler;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use tauri_plugin_shell::ShellExt;

    async fn get_shell_version(handle: &tauri::AppHandle) -> Result<String, String> {
        let shell = handle.shell();
        let output = shell
            .command("helm")
            .args(["version", "--short"])
            .output()
            .await
            .or(Err("Command execution failed.".into()))
            .and_then(|out| {
                if out.status.success() {
                    Ok(String::from_utf8(out.stdout).unwrap())
                } else {
                    Err(format!(
                        "Command failed with code {}",
                        out.status.code().unwrap_or(-1)
                    ))
                }
            });
        output
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(tag = "command")]
    pub enum HelmCommand {
        GetVersion {},
    }

    impl CommandHandler for HelmCommand {
        async fn execute(&self, handle: &tauri::AppHandle) -> Result<Value, String> {
            match *self {
                HelmCommand::GetVersion {} => self.wrap_in_value(get_shell_version(handle).await),
            }
        }
    }
}
