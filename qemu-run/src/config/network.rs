extern crate sha2;

use std::borrow::Cow;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Network {
    Bridge { bridge: String },
}

impl Network {
    pub fn gen_params(&self, name: &str, index: usize) -> Vec<Cow<str>> {
        let macaddr = {
            use self::sha2::Digest;
            let digest = sha2::Sha256::digest_str(&format!("{}:{}", name, index));
            format!("52:54:{:02x}:{:02x}:{:02x}:{:02x}",
                    digest[0],
                    digest[1],
                    digest[2],
                    digest[3])
        };
        let mut params = vec_from!["-net", format!("nic,vlan={},macaddr={}", index, macaddr)];
        match self {
            &Network::Bridge { ref bridge } => {
                params.extend(vec_from!["-net", format!("bridge,vlan={},br={}", index, bridge)])
            }
        }
        params
    }
}
