pub mod kube_api {
    use serde::{Deserialize, Serialize};
    use crate::CommandHandler;
    
    #[derive(Serialize, Deserialize, Clone)]
    pub enum KubeCommand {}
    impl CommandHandler for KubeCommand {}
}