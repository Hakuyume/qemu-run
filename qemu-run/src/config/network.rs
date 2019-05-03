use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::borrow;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Network {
    Bridge { bridge: String },
}

impl Network {
    pub fn gen_params(&self, name: &str, index: usize) -> Vec<borrow::Cow<str>> {
        let mac = {
            let digest = Sha256::digest(format!("{}:{}", name, index).as_bytes());
            format!(
                "52:54:{:02x}:{:02x}:{:02x}:{:02x}",
                digest[0], digest[1], digest[2], digest[3]
            )
        };
        let mut params = vec_from!["-device", format!("e1000,netdev=net{},mac={}", index, mac)];
        match self {
            &Network::Bridge { ref bridge } => params.extend(vec_from![
                "-netdev",
                format!("bridge,id=net{},br={}", index, bridge)
            ]),
        }
        params
    }
}

#[cfg(test)]
mod tests {
    use super::Network;
    use serde_yaml;

    #[test]
    fn bridge() {
        let network: Network = serde_yaml::from_str("{bridge: br0}").unwrap();
        assert_eq!(
            network.gen_params("name", 0),
            [
                "-device",
                "e1000,netdev=net0,mac=52:54:4a:a3:57:c1",
                "-netdev",
                "bridge,id=net0,br=br0"
            ]
        );
    }

    #[test]
    #[should_panic]
    fn unknown() {
        let _: Network = serde_yaml::from_str("{unknown: unknown}").unwrap();
    }
}
