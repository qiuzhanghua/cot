use clap::{command, Arg, Command};
use log::{debug, error, info, trace, warn};
use log4rs::{self, config::RawConfig};
use rust_embed::Embed;
use std::env;

#[derive(Embed)]
#[folder = "static/"]
#[prefix = ""]
struct Asset;

fn main() {
    // for file in Asset::iter() {
    //     println!("{}", file.as_ref());
    // }
    // use current directory log4rs.yml if exists
    let mut init_log = log4rs::init_file("log4rs.yml", Default::default());
    // otherwise use log4rs.yaml in same directory as executable
    if init_log.is_err() {
        if let Ok(exe_path) = env::current_exe() {
            let exe_dir = exe_path.parent().unwrap();
            let log4rs_yml = exe_dir.join("log4rs.yml");
            init_log = log4rs::init_file(log4rs_yml, Default::default());
        }
    }
    if init_log.is_err() {
        let log4rs_yaml = Asset::get("log4rs.yaml").unwrap();
        let log4rs_yaml_str = std::str::from_utf8(log4rs_yaml.data.as_ref()).unwrap();
        let config: RawConfig = serde_yaml::from_str(log4rs_yaml_str).unwrap();
        log4rs::init_raw_config(config).unwrap();
    }
    // set logging level to off default
    // if LOGGING_LEVEL is set in environment, use that
    let logging_level = env::var("LOGGING_LEVEL").unwrap_or("off".to_string());
    let logging_level = match logging_level.to_lowercase().as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Off,
    };
    log::set_max_level(logging_level);

    let mut cmd = command!()
        .help_template(
            "{about}
{author}

Commands:
{subcommands}",
        )
        .subcommand(
            Command::new("install")
                .about("Install/add/i Cot or plugins")
                .aliases(["add", "i"]),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove/delete/del/rm plugin")
                .aliases(["rm", "delete", "del"]),
        )
        .subcommand(
            Command::new("use")
                .about("Use/active plugin")
                .aliases(["active"]),
        )
        .subcommand(
            Command::new("list")
                .about("List/ls plugins")
                .aliases(["ls"]),
        )
        .subcommand(
            Command::new("init")
                .about("Init/create new project")
                .aliases(["create"]),
        )
        .subcommand(
            Command::new("huggingface")
                .about("Huggingface/hf directory")
                .aliases(["hf"]),
        )
        .subcommand(
            Command::new("huggingface-datasets")
                .about("Huggingface-datasets/hd <id>'s directory")
                .aliases(["hd"]),
        )
        .subcommand(
            Command::new("huggingface-models")
                .about("Huggingface-models/hm <id>'s directory")
                .aliases(["hm"]),
        )
        .subcommand(
            Command::new("xf")
                .about("Extract <filename.tar.gz>")
                .arg(Arg::new("filename")),
        )
        .subcommand(
            Command::new("unzip")
                .about("Extract <filename.zip>")
                .arg(Arg::new("filename")),
        )
        .subcommand(
            Command::new("tag")
                .about("Tag [current|next|write|date|hash|show]")
                .aliases(["t"])
                .subcommand(Command::new("current").about("Current tag").aliases(["c"]))
                .subcommand(
                    Command::new("next")
                        .about("Next [major|minor|patch|pre|phase] of current tag")
                        .aliases(["n"]),
                )
                .subcommand(
                    Command::new("write")
                        .about("Write current tag info into file")
                        .aliases(["w"]),
                )
                .subcommand(
                    Command::new("date")
                        .about("Date of current tag")
                        .aliases(["d"]),
                )
                .subcommand(
                    Command::new("hash")
                        .about("Hash of current tag")
                        .aliases(["h"]),
                )
                .subcommand(
                    Command::new("show")
                        .about("Show information about tag")
                        .aliases(["s"]),
                ),
        );
    let matches = cmd.clone().get_matches();
    let subcommand = matches.subcommand();
    match subcommand {
        None => {
            let _ = cmd.print_help();
        }
        Some((cmd_name, args)) => match cmd_name {
            "xf" => {
                let filename = args.get_one::<String>("filename");
                trace!("xf filename: {:?}", filename);
            }
            _ => {
                trace!("cmd_name: {:?}, args: {:?}", cmd_name, args);
            }
        },
    }
}
