pub mod artifacts_api {
    use serde::{Deserialize, Serialize};
    use crate::CommandHandler;

    #[derive(Serialize, Deserialize, Clone)]
    pub enum ArtifactsCommand {}
    impl CommandHandler for ArtifactsCommand {}
}