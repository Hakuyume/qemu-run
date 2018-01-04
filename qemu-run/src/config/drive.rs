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
                &Format::Raw => "raw",
            };
            param += format!(",format={}", format).as_str();
        }
        if self.discard {
            param += ",discard=on";
        }
        vec_from!["-drive", param]
    }
}
