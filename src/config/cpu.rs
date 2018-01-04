use std::borrow::Cow;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cpu {
    #[serde(default)]
    kvm: bool,
    #[serde(rename = "type")]
    type_: Option<String>,
    cores: Option<usize>,
}

impl Cpu {
    pub fn gen_params(&self) -> Vec<Cow<str>> {
        let mut params = Vec::new();
        if self.kvm {
            params.push("-enable-kvm".into());
        }
        if let Some(ref type_) = self.type_ {
            params.extend(vec_from!["-cpu", type_.as_str()])
        }
        if let Some(cores) = self.cores {
            params.extend(vec_from!["-smp", format!("sockets=1,cores={}", cores)]);
        }
        params
    }
}
