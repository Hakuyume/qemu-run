use std::borrow;

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
    pub fn gen_params(&self) -> Vec<borrow::Cow<str>> {
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

#[cfg(test)]
mod tests {
    use serde_yaml;
    use super::Cpu;

    #[test]
    fn default() {
        assert!(Cpu::default().gen_params().is_empty());
    }

    #[test]
    fn all() {
        let cpu: Cpu = serde_yaml::from_str("{kvm: true, type: core2duo, cores: 2}").unwrap();
        assert_eq!(cpu.gen_params(),
                   ["-enable-kvm",
                    "-cpu",
                    "core2duo",
                    "-smp",
                    "sockets=1,cores=2"]);
    }

    #[test]
    #[should_panic]
    fn unknown() {
        let _: Cpu = serde_yaml::from_str("{unknown: unknown}").unwrap();
    }
}
