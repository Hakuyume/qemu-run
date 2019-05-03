use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Drive {
    file: String,
    format: Option<Format>,
    #[serde(default)]
    discard: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Format {
    Raw,
}

impl Drive {
    pub fn gen_params(&self) -> Vec<Cow<str>> {
        let mut param = format!("file={}", self.file);
        if let Some(ref format) = self.format {
            let format = match format {
                Format::Raw => "raw",
            };
            param += format!(",format={}", format).as_str();
        }
        if self.discard {
            param += ",discard=on";
        }
        vec_from!["-drive", param]
    }
}

#[cfg(test)]
mod tests {
    use super::Drive;
    use serde_yaml;

    #[test]
    fn default() {
        let drive: Drive = serde_yaml::from_str("{file: /dev/sda}").unwrap();
        assert_eq!(drive.gen_params(), ["-drive", "file=/dev/sda"]);
    }

    #[test]
    fn all() {
        let drive: Drive =
            serde_yaml::from_str("{file: /dev/sda, format: raw, discard: true}").unwrap();
        assert_eq!(
            drive.gen_params(),
            ["-drive", "file=/dev/sda,format=raw,discard=on"]
        );
    }

    #[test]
    #[should_panic]
    fn unknown() {
        let _: Drive = serde_yaml::from_str("{unknown: unknown}").unwrap();
    }

    #[test]
    #[should_panic]
    fn unknown_format() {
        let _: Drive = serde_yaml::from_str("{file: /dev/sda, format: unknown}").unwrap();
    }
}
