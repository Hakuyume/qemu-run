extern crate rand;

use std::borrow::Cow;

macro_rules! vec_from {
    ($($x:expr),*) => {
        vec![$($x.into(),)*]
    }
}

mod cpu;
mod drive;
mod network;
mod rtc;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default = "default_name")]
    name: String,
    #[serde(default)]
    uefi: bool,
    #[serde(default)]
    cpu: cpu::Cpu,
    memory: Option<String>,
    #[serde(default)]
    drive: Vec<drive::Drive>,
    #[serde(default)]
    network: Vec<network::Network>,
    #[serde(default)]
    spice: bool,
    #[serde(default)]
    sound: bool,
    #[serde(default)]
    spice_guest: bool,
    #[serde(default)]
    rtc: rtc::Rtc,
    #[serde(default)]
    option: Vec<Vec<String>>,
}

impl Config {
    pub fn gen_params(&self) -> Vec<Cow<str>> {
        let mut params = vec_from!["-name",
                                   self.name.as_str(),
                                   "-monitor",
                                   format!("unix:/tmp/qemu-monitor-{}.sock,server,nowait",
                                           self.name)];
        if self.uefi {
            params.extend(vec_from!["-bios", "/usr/share/ovmf/ovmf_code_x64.bin"]);
        }
        params.extend(self.cpu.gen_params());
        if let Some(ref memory) = self.memory {
            params.extend(vec_from!["-m", memory.as_str()]);
        }
        for drive in self.drive.iter() {
            params.extend(drive.gen_params());
        }
        for (i, network) in self.network.iter().enumerate() {
            params.extend(network.gen_params(&self.name, i));
        }
        if self.spice {
            params.extend(vec_from!["-vga",
                                    "qxl",
                                    "-spice",
                                    format!("disable-ticketing,unix,addr=/tmp/qemu-spice-{}.sock",
                                            self.name)]);
        }
        if self.sound {
            params.extend(vec_from!["-device", "intel-hda", "-device", "hda-micro"]);
        }
        if self.spice_guest {
            params.extend(vec_from!["-device",
                                    "virtio-serial-pci",
                                    "-device",
                                    "virtserialport,chardev=spicechannel0,name=com.redhat.spice.0",
                                    "-chardev",
                                    "spicevmc,id=spicechannel0,name=vdagent"]);
        }
        params.extend(self.rtc.gen_params());
        for option in self.option.iter() {
            params.extend(option.iter().map(|s| s.as_str().into()));
        }
        params
    }
}

fn default_name() -> String {
    use self::rand::Rng;
    let mut rng = rand::thread_rng();
    format!("vm-{:04x}", rng.gen::<u16>())
}