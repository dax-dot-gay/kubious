pub mod application_api {
    use serde::{Deserialize, Serialize};
    use crate::CommandHandler;

    #[derive(Serialize, Deserialize, Clone)]
    pub enum ApplicationCommand {}
    impl CommandHandler for ApplicationCommand {}
}

mod state;
pub use state::app_state;