pub mod kompose_api {
    use serde::{Deserialize, Serialize};
    use crate::CommandHandler;

    #[derive(Serialize, Deserialize, Clone)]
    pub enum KomposeCommand {}
    impl CommandHandler for KomposeCommand {}
}