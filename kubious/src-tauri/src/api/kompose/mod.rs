pub mod kompose_api {
    use serde::{Deserialize, Serialize};
    use crate::CommandHandler;

    use tauri_plugin_shell::ShellExt;

    fn get_shell_version(handle: &tauri::AppHandle) -> Result<String, String> {
        let shell = handle.shell();
        let output = tauri::async_runtime::block_on(async move {
            shell
                .command("helm")
                .args(["version", "--short"])
                .output()
                .await
        })
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

    #[derive(Serialize, Deserialize, Clone)]
    pub enum KomposeCommand {
        GetVersion
    }
    impl CommandHandler for KomposeCommand {
        fn execute(&self, handle: &tauri::AppHandle) -> Result<impl Serialize, String> {
            match *self {
                KomposeCommand::GetVersion => get_shell_version(handle)
            }
        }
    }
}