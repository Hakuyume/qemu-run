use std::borrow::Cow;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rtc {
    base: Option<Base>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Base {
    Localtime,
}

impl Rtc {
    pub fn gen_params(&self) -> Vec<Cow<str>> {
        if let Some(ref base) = self.base {
            let base = match base {
                &Base::Localtime => "localtime",
            };
            vec_from!["-rtc", format!("base={}", base)]
        } else {
            Vec::new()
        }
    }
}
