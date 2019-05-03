mod config;

use config::Config;
use docopt::Docopt;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

const USAGE: &str = "
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
    arg_config: PathBuf,
}

fn main() {
    let mut command = {
        let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        let name = match args.flag_name {
            Some(ref name) => name.as_str(),
            None => args.arg_config.file_stem().unwrap().to_str().unwrap(),
        };

        let config: Config = {
            let reader = File::open(&args.arg_config).unwrap();
            serde_yaml::from_reader(reader).unwrap()
        };

        let params = config.gen_params(name).unwrap();
        if args.flag_dry_run {
            println!("{}", params.join(" "));
            return;
        }

        let mut command = Command::new("qemu-system-x86_64");
        command.args(params.iter().map(AsRef::as_ref));
        command
    };

    panic!("{}", command.exec().description());
}
