pub mod app_state {
    use std::{collections::HashMap, sync::{Mutex, MutexGuard}};
    use kube::Config;
    use serde::{Deserialize, Serialize};

    use crate::compat::kube_compat::KubeConfig;


    #[derive(Serialize, Deserialize)]
    pub struct AppState {
        configs: Mutex<HashMap<String, KubeConfig>>,
        current_config: Mutex<Option<String>>
    }

    impl AppState {
        fn configs_mutable(&self) -> MutexGuard<HashMap<String, KubeConfig>> {
            if let Ok(locked) = self.configs.lock() {
                locked
            } else {
                panic!("Failed to lock state.configs!");
            }
        }

        fn current_config_mutable(&self) -> MutexGuard<Option<String>> {
            if let Ok(locked) = self.current_config.lock() {
                locked
            } else {
                panic!("Failed to lock state.current_config!");
            }
        }

        pub fn set_current_config(&self, value: Option<String>) -> Result<Option<KubeConfig>, String> {
            let mut current = self.current_config_mutable();
            if let Some(name) = value {
                if let Some(c) = self.configs_mutable().get(name.as_str()) {
                    *current = Some(name);
                    Ok(Some(c.clone()))
                } else {
                    Err("Unknwon config name".to_string())
                }
            } else {
                Ok(None)
            }
        }

        pub fn get_current_config(&self) -> Option<KubeConfig> {
            if let Some(current) = self.current_config_mutable().clone() {
                if let Some(c) = self.configs_mutable().get(&current) {
                    return Some(c.clone());
                }
            }
            None
        }

        pub fn get_configs(&self) -> HashMap<String, KubeConfig> {
            self.configs_mutable().clone()
        }
        
        pub fn put_config(&self, key: &str, config: Config) -> KubeConfig {
            let mut configs = self.configs_mutable();
            let converted = KubeConfig::from(config);
            (*configs).insert(key.to_string(), converted.clone());
            converted.clone()
        }

        pub fn put_kubeconfig(&self, key: &str, config: KubeConfig) -> KubeConfig {
            let mut configs = self.configs_mutable();
            (*configs).insert(key.to_string(), config.clone());
            config.clone()
        }

        pub fn remove_config(&self, key: &str) {
            let mut configs = self.configs_mutable();
            let current = self.current_config_mutable();
            if let Some(ck) = current.clone() {
                if ck == key.to_string() {
                    let _ = self.set_current_config(None);
                }
            }
            (*configs).remove(key);
        }

        pub async fn register_default(&self) -> Option<KubeConfig> {
            if let Ok(inferred) = Config::infer().await {
                self.put_config("default", inferred.clone());
                Some(KubeConfig::from(inferred))
            } else {
                None
            }
        }
    }
}