pub mod kube_api {
    use serde::{Deserialize, Serialize};
    use crate::CommandHandler;
    
    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(tag = "command")]
    pub enum KubeCommand {}
    impl CommandHandler for KubeCommand {}
}