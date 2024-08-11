pub mod helm_api {
    use serde::{Deserialize, Serialize};
    use crate::CommandHandler;

    #[derive(Serialize, Deserialize, Clone)]
    pub enum HelmCommand {}
    impl CommandHandler for HelmCommand {}
}