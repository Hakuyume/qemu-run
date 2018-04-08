extern crate rand;

use std::borrow::Cow;
use std::error;
use std::fs;
use std::path;

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
        let mut params =
            vec_from!["-name",
                      self.name.as_str(),
                      "-monitor",
                      format!("unix:/tmp/qemu/{}/monitor.sock,server,nowait", self.name),
                      "-serial",
                      format!("unix:/tmp/qemu/{}/serial.sock,server,nowait", self.name)];
        if self.uefi {
            params.extend(vec_from!["-drive",
                                    "if=pflash,format=raw,readonly,file=/usr/share/ovmf/x64/OVMF_CODE.fd",
                                    "-drive",
                                    format!("if=pflash,format=raw,file=/var/lib/qemu/{}/OVMF_VARS.fd",
                                            self.name)]);
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
                                    format!("disable-ticketing,unix,addr=/tmp/qemu/{}/spice.sock",
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

    pub fn prepare(&self) -> Result<(), Box<error::Error>> {
        let sock_dir = path::PathBuf::from(format!("/tmp/qemu/{}", self.name));
        fs::create_dir_all(&sock_dir)?;
        for sock in &["monitor", "serial", "spice"] {
            let _ = fs::remove_file(sock_dir.join(format!("{}.sock", sock)));
        }

        if self.uefi {
            let ovmf_vars = path::PathBuf::from(format!("/var/lib/qemu/{}/OVMF_VARS.fd",
                                                        self.name));
            if !ovmf_vars.exists() {
                fs::create_dir_all(ovmf_vars.parent().unwrap())?;
                fs::copy("/usr/share/ovmf/x64/OVMF_VARS.fd", ovmf_vars)?;
            }
        }
        Ok(())
    }
}

fn default_name() -> String {
    use self::rand::Rng;
    let mut rng = rand::thread_rng();
    format!("vm-{:04x}", rng.gen::<u16>())
}

#[cfg(test)]
mod tests {
    use serde_yaml;
    use super::Config;

    #[test]
    fn readme() {
        let config: Config = serde_yaml::from_str(r#"
name: guest
uefi: true
cpu:
  kvm: true
  type: host
  cores: 2
memory: 4G
drive:
  - file: /dev/sdb
    format: raw
network:
  - bridge: br0
  - bridge: br1
"#)
                .unwrap();
        assert_eq!(config.gen_params(),
                   ["-name",
                    "guest",
                    "-monitor",
                    "unix:/tmp/qemu/guest/monitor.sock,server,nowait",
                    "-serial",
                    "unix:/tmp/qemu/guest/serial.sock,server,nowait",
                    "-drive",
                    "if=pflash,format=raw,readonly,file=/usr/share/ovmf/x64/OVMF_CODE.fd",
                    "-drive",
                    "if=pflash,format=raw,file=/var/lib/qemu/guest/OVMF_VARS.fd",
                    "-enable-kvm",
                    "-cpu",
                    "host",
                    "-smp",
                    "sockets=1,cores=2",
                    "-m",
                    "4G",
                    "-drive",
                    "file=/dev/sdb,format=raw",
                    "-device",
                    "e1000,netdev=net0,mac=52:54:ff:be:28:bc",
                    "-netdev",
                    "bridge,id=net0,br=br0",
                    "-device",
                    "e1000,netdev=net1,mac=52:54:a9:4c:0b:80",
                    "-netdev",
                    "bridge,id=net1,br=br1"]);
    }

    #[test]
    #[should_panic]
    fn unknown() {
        let _: Config = serde_yaml::from_str("{unknown: unknown}").unwrap();
    }
}
