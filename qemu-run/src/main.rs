extern crate docopt;
extern crate libusb;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::fs;
use std::path;
use std::process;

mod config;

const USAGE: &'static str = "
Usage:
  qemu-run [-d | --dry-run] [--name <name>] <config>

Options:
  -d --dry-run  Show generated params
  --name        Name of machine
";

#[derive(Deserialize)]
struct Args {
    flag_dry_run: bool,
    flag_name: Option<String>,
    arg_config: path::PathBuf,
}

fn main() {
    let mut command = {
        let args: Args = docopt::Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        let name = match args.flag_name {
            Some(ref name) => name.as_str(),
            None => args.arg_config.file_stem().unwrap().to_str().unwrap(),
        };

        let config: config::Config = {
            let reader = fs::File::open(&args.arg_config).unwrap();
            serde_yaml::from_reader(reader).unwrap()
        };

        let params = config.gen_params(name).unwrap();
        if args.flag_dry_run {
            println!("{}", params.join(" "));
            return;
        }

        let mut command = process::Command::new("qemu-system-x86_64");
        command.args(params.iter().map(|p| p.as_ref()));
        command
    };

    use std::error::Error;
    use std::os::unix::process::CommandExt;
    panic!("{}", command.exec().description());
}
