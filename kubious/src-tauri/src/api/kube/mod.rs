pub mod kube_api {
    use crate::{api::app_state::AppState, CommandHandler};
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup;
    use serde::{Deserialize, Serialize};
    use tauri::Manager;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(tag = "command")]
    pub enum KubeCommand {
        SupportedGroups {},
        SupportedResources { group: APIGroup }
    }
    impl CommandHandler for KubeCommand {
        async fn execute(&self, handle: &tauri::AppHandle) -> Result<serde_json::Value, String> {
            if let Some(client) = handle.state::<AppState>().client().await {
                match self {
                    KubeCommand::SupportedGroups {} => {
                        if let Ok(groups) = client.list_api_groups().await {
                            let mut all_groups = groups.groups.clone();
                            all_groups.push(APIGroup {
                                name: "core".to_string(),
                                preferred_version: None,
                                server_address_by_client_cidrs: None,
                                versions: Vec::new()
                            });
                            self.wrap_in_value(Ok(all_groups))
                        } else {
                            Err("Failed to list groups.".to_string())
                        }
                    }
                    KubeCommand::SupportedResources { group } => {
                        if group.name == "core".to_string() {
                            if let Ok(versions) = client.list_core_api_versions().await {
                                if let Some(version) = versions.versions.first() {
                                    if let Ok(resources) = client.list_core_api_resources(version).await
                                    {
                                        self.wrap_in_value(Ok(resources.resources))
                                    } else {
                                        Err("Failed to list resources.".to_string())
                                    }
                                } else {
                                    Err("No valid versions".to_string())
                                }
                            } else {
                                Err("Failed to fetch API version".to_string())
                            }
                        } else {
                            if let Ok(resources) = client
                                .list_api_group_resources(
                                    group
                                        .preferred_version
                                        .as_ref()
                                        .or_else(|| {
                                            Some(group.versions.first().expect("existing versions"))
                                        })
                                        .unwrap()
                                        .version
                                        .as_str(),
                                )
                                .await
                            {
                                self.wrap_in_value(Ok(resources.resources))
                            } else {
                                Err("Failed to list resources.".to_string())
                            }
                        }
                        
                    }
                    
                }
            } else {
                Err("Could not establish connection.".to_string())
            }
        }
    }
}
