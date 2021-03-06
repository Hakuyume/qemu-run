use libusb::{Context, Error};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Default, Deserialize)]
pub struct Usb(Option<Vec<(u16, u16)>>);

impl Usb {
    pub fn gen_params(&self) -> Result<Vec<Cow<'_, str>>, Error> {
        if let Some(ref ids) = self.0 {
            let mut params = vec_from!["-device", "nec-usb-xhci,id=xhci"];

            let context = Context::new()?;
            for device in context.devices()?.iter() {
                if let Ok(desc) = device.device_descriptor() {
                    if ids
                        .iter()
                        .any(|id| &(desc.vendor_id(), desc.product_id()) == id)
                    {
                        params.extend(vec_from![
                            "-device",
                            format!(
                                "usb-host,id=usb{0}_{1},bus=xhci.0,hostbus={0},hostaddr={1}",
                                device.bus_number(),
                                device.address()
                            )
                        ]);
                    }
                }
            }
            Ok(params)
        } else {
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Usb;
    use serde_yaml;

    #[test]
    fn default() {
        assert!(Usb::default().gen_params().unwrap().is_empty());
    }

    #[test]
    fn empty() {
        let usb: Usb = serde_yaml::from_str("[]").unwrap();
        assert_eq!(
            usb.gen_params().unwrap(),
            ["-device", "nec-usb-xhci,id=xhci"]
        );
    }

    #[test]
    fn parse() {
        let _: Usb = serde_yaml::from_str("[[0x0123, 0x4567], [0x89ab, 0xcdef]]").unwrap();
    }
}
