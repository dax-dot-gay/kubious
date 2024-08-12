pub mod application_api {
    use serde::{Deserialize, Serialize};
    use crate::CommandHandler;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(tag = "command")]
    pub enum ApplicationCommand {
        SetCurrentConfig{key: String},
        GetCurrentConfig{},
        GetConfigs{}
    }
    impl CommandHandler for ApplicationCommand {}
}

mod state;
pub use state::app_state;