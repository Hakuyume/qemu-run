use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rtc {
    base: Option<Base>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Base {
    Utc,
    Localtime,
}

impl Rtc {
    pub fn gen_params(&self) -> Vec<Cow<str>> {
        if let Some(ref base) = self.base {
            let base = match base {
                Base::Utc => "utc",
                Base::Localtime => "localtime",
            };
            vec_from!["-rtc", format!("base={}", base)]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rtc;
    use serde_yaml;

    #[test]
    fn default() {
        assert!(Rtc::default().gen_params().is_empty());
    }

    #[test]
    fn all() {
        let rtc: Rtc = serde_yaml::from_str("{base: localtime}").unwrap();
        assert_eq!(rtc.gen_params(), ["-rtc", "base=localtime"]);
    }

    #[test]
    #[should_panic]
    fn unknown() {
        let _: Rtc = serde_yaml::from_str("{unknown: unknown}").unwrap();
    }

    #[test]
    #[should_panic]
    fn unknown_base() {
        let _: Rtc = serde_yaml::from_str("{base: unknown}").unwrap();
    }
}
