#[macro_use]
extern crate serde_derive;

extern crate docopt;
extern crate serde;
extern crate serde_yaml;

use std::fs;
use std::process;

mod config;

const USAGE: &'static str = "
Usage:
  qemu-run [-d | --dry-run] <config>

Options:
  -d --dry-run  Show generated params
";

#[derive(Deserialize)]
struct Args {
    flag_dry_run: bool,
    arg_config: String,
}

fn main() {
    let mut command = {
        let args: Args = docopt::Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        let config: config::Config = {
            let reader = fs::File::open(args.arg_config).unwrap();
            serde_yaml::from_reader(reader).unwrap()
        };

        let params = config.gen_params();
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
