use libusb;
use std::borrow;

#[derive(Debug, Default, Deserialize)]
pub struct Usb(Vec<(u16, u16)>);

impl Usb {
    pub fn gen_params(&self) -> Result<Vec<borrow::Cow<str>>, libusb::Error> {
        if !self.0.is_empty() {
            let mut params = vec_from!["-device", "nec-usb-xhci,id=xhci"];

            let context = libusb::Context::new()?;
            for device in context.devices()?.iter() {
                if let Ok(desc) = device.device_descriptor() {
                    if self.0
                           .iter()
                           .any(|id| &(desc.vendor_id(), desc.product_id()) == id) {
                        params.extend(vec_from!["-device", format!(
                            "usb-host,id=usb{0}_{1},bus=xhci.0,hostbus={0},hostaddr={1}",
                            device.bus_number(), device.address())]);
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
    use serde_yaml;
    use super::Usb;

    #[test]
    fn default() {
        assert!(Usb::default().gen_params().unwrap().is_empty());
    }

    #[test]
    fn parse() {
        let _: Usb = serde_yaml::from_str("[[0x0123, 0x4567], [0x89ab, 0xcdef]]").unwrap();
    }
}