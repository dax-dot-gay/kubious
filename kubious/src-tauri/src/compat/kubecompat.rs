pub mod kube_compat {
    use http::{
        HeaderName, HeaderValue, Uri,
    };
    use std::{str::FromStr, time::Duration};

    use kube::{config::AuthInfo, Config};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct KubeConfig {
        pub cluster_url: String,
        pub default_namespace: String,
        pub root_cert: Option<Vec<Vec<u8>>>,
        pub connect_timeout: Option<Duration>,
        pub read_timeout: Option<Duration>,
        pub write_timeout: Option<Duration>,
        pub accept_invalid_certs: bool,
        pub auth_info: AuthInfo,
        pub proxy_url: Option<String>,
        pub tls_server_name: Option<String>,
        pub headers: Vec<(String, Option<String>)>,
    }

    impl From<Config> for KubeConfig {
        fn from(value: Config) -> Self {
            KubeConfig {
                cluster_url: value.cluster_url.to_string(),
                default_namespace: value.default_namespace,
                root_cert: value.root_cert,
                connect_timeout: value.connect_timeout,
                read_timeout: value.read_timeout,
                write_timeout: value.write_timeout,
                accept_invalid_certs: value.accept_invalid_certs,
                auth_info: value.auth_info,
                proxy_url: match value.proxy_url {
                    Some(p) => Some(p.to_string()),
                    None => None,
                },
                tls_server_name: value.tls_server_name,
                headers: value
                    .headers
                    .iter()
                    .map(|(name, val)| {
                        (
                            name.to_string(),
                            val.to_str().and_then(|v| Ok(v.to_string())).ok(),
                        )
                    })
                    .collect(),
            }
        }
    }

    impl Into<Config> for KubeConfig {
        fn into(self) -> Config {

            Config {
                cluster_url: Uri::from_str(self.cluster_url.as_str()).expect("URI Parsing failed"),
                default_namespace: self.default_namespace,
                root_cert: self.root_cert,
                read_timeout: self.read_timeout,
                connect_timeout: self.connect_timeout,
                write_timeout: self.write_timeout,
                accept_invalid_certs: self.accept_invalid_certs,
                auth_info: self.auth_info,
                proxy_url: match self.proxy_url {
                    Some(p) => Some(Uri::from_str(p.as_str()).expect("URI Parsing failed")),
                    None => None,
                },
                tls_server_name: self.tls_server_name,
                headers: self.headers.clone().into_iter().map(|(name, val)| {
                    (
                        HeaderName::from_str(name.as_str()).unwrap(),
                        val.and_then(|v| Some(HeaderValue::from_str(v.as_str()).unwrap()))
                            .or_else(|| Some(HeaderValue::from_str("").unwrap()))
                            .unwrap(),
                    )
                }).collect(),
            }
        }
    }
}
