use crate::util::{git_add_tag, next_major, next_minor, next_patch, next_phase, next_pre};
use clap::{command, Arg, Command};
use log::{error, trace, warn};
use log4rs::{self, config::RawConfig};
use rust_embed::Embed;
use semver::Version;
use std::env;
use std::io;
use std::path::PathBuf;

mod util;

#[derive(Embed)]
#[folder = "static/"]
#[prefix = ""]
struct Asset;

fn main() -> anyhow::Result<()> {
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
        log4rs::init_raw_config(config)?;
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
                .aliases(["hd"])
                .arg(Arg::new("id").help("datasets id, such as 'sentence-transformers/all-nli'")),
        )
        .subcommand(
            Command::new("huggingface-models")
                .about("Huggingface-models/hm <id>'s directory")
                .aliases(["hm"])
                .arg(Arg::new("id").help("model id, such as 'baai/bge-large-zh-v1.5'")),
        )
        .subcommand(
            Command::new("xf")
                .about("Extract <filename.tar.gz>")
                .arg(
                    Arg::new("directory")
                        .short('C')
                        .long("Extract <filename.tar.gz> into <directory>")
                        .default_value("."),
                )
                .arg(Arg::new("filename")),
        )
        .subcommand(
            Command::new("unzip")
                .about("Extract <filename.zip>")
                .arg(
                    Arg::new("directory")
                        .short('d')
                        .long("Extract <filename.zip> into <directory>")
                        .default_value("."),
                )
                .arg(Arg::new("filename")),
        )
        .subcommand(
            Command::new("tag")
                .about("Tag [current|next|date|hash|show]")
                .aliases(["t"])
                .subcommand(Command::new("current").about("Current tag").aliases(["c"]))
                .subcommand(
                    Command::new("next")
                        .about("Next [major|minor|patch|pre|phase] of current tag")
                        .aliases(["n"])
                        .subcommand(
                            Command::new("major")
                                .about("Next major version of current tag")
                                .aliases(["a"]),
                        )
                        .subcommand(
                            Command::new("minor")
                                .about("Next minor version of current tag")
                                .aliases(["i"]),
                        )
                        .subcommand(
                            Command::new("patch")
                                .about("Next patch version of current tag")
                                .aliases(["p"]),
                        )
                        .subcommand(
                            Command::new("phase")
                                .about("Next phase version of current tag")
                                .aliases(["s"]),
                        )
                        .subcommand(
                            Command::new("pre")
                                .about("Next pre version of current tag")
                                .aliases(["r"]),
                        ),
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
            Ok(())
        }
        Some((cmd_name, args)) => match cmd_name {
            "huggingface" => {
                let hf_home = util::hf_home()?;
                print!("{}", hf_home);
                Ok(())
            }
            "huggingface-models" => {
                let id = args.get_one::<String>("id");
                if id.is_none() {
                    warn!("model id is required, such as 'baai/bge-large-zh-v1.5'");
                    return Ok(());
                }
                let hf_model = util::hf_model_path(id.unwrap())?;
                print!("{}", hf_model);
                Ok(())
            }
            "huggingface-datasets" => {
                let id = args.get_one::<String>("id");
                if id.is_none() {
                    warn!("datasets id is required, such as 'sentence-transformers/all-nli'");
                    return Ok(());
                }
                let hf_datasets = util::hf_datasets_path(id.unwrap())?;
                print!("{}", hf_datasets);
                Ok(())
            }
            "xf" => {
                let filename = args.get_one::<String>("filename");
                let directory = args.get_one::<String>("directory");
                trace!("xf {:?} -> {:?}", filename, directory);
                let file = PathBuf::from(filename.unwrap());
                let working_dir = directory.unwrap();
                if file.extension().is_none() {
                    warn!("filename {:?} does not have an extension", filename);
                    return Ok(());
                }
                let ext = file.extension().unwrap().to_str().unwrap();
                match ext {
                    "gz" => {
                        // println!("Extracting {:?} -> {}", file, working_dir);
                        let stem = file.file_stem().unwrap().to_string_lossy().into_owned();
                        if !stem.ends_with(".tar") {
                            warn!("filename {:?} does not have a .tar.gz extension", filename);
                            return Ok(());
                        }
                        let temp_dir = tempfile::tempdir()?;
                        let tar_file = temp_dir.path().join(stem);
                        util::decompress(file.to_str().unwrap(), tar_file.to_str().unwrap())?;
                        util::extract(tar_file.to_str().unwrap(), working_dir)?;
                    }
                    "tgz" => {
                        // trace!("Extracting {:?} -> {}", file, directory);
                        let stem = file.file_stem().unwrap().to_string_lossy().into_owned();
                        let temp_dir = tempfile::tempdir()?;
                        let tar_file = temp_dir.path().join(stem + ".tar");
                        util::decompress(file.to_str().unwrap(), tar_file.to_str().unwrap())?;
                        util::extract(tar_file.to_str().unwrap(), working_dir)?;
                    }
                    "tar" => {
                        // trace!("Extracting {:?} -> {}", file, working_dir);
                        util::extract(file.to_str().unwrap(), working_dir)?;
                    }
                    _ => {
                        warn!("unknown extension {:?}", ext);
                    }
                }
                Ok(())
            }
            "unzip" => {
                let filename = args.get_one::<String>("filename");
                let directory = args.get_one::<String>("directory");
                let file = PathBuf::from(filename.unwrap());
                let working_dir = directory.unwrap();
                if file.extension().is_none() {
                    warn!("filename {:?} does not have an extension", filename);
                }
                let ext = file.extension().unwrap().to_str().unwrap();
                if ext == "zip" {
                    util::unzip(file.to_str().unwrap(), working_dir)?;
                } else {
                    warn!("unknown extension {:?}", ext);
                }
                Ok(())
            }
            "tag" => {
                let tag_cmd = args.subcommand();
                match tag_cmd {
                    None => Ok(()),
                    Some((cmd_name, _)) => match cmd_name {
                        "current" => {
                            let tag = util::git_latest_tag()?;
                            print!("{}", tag);
                            Ok(())
                        }
                        "next" => {
                            let tag_next_cmd = args.subcommand();
                            match tag_next_cmd {
                                None => {
                                    let _ = cmd.print_help();
                                    Ok(())
                                }
                                Some((_, matches2)) => {
                                    let cmd_tag_next = matches2.subcommand();
                                    match cmd_tag_next {
                                        None => Ok(()),
                                        Some((cmd_name, _)) => match cmd_name {
                                            "major" => {
                                                let tag = util::git_latest_tag()?;
                                                let version = Version::parse(&tag)?;
                                                let next_version = next_major(version);
                                                match git_add_tag(&next_version.to_string()) {
                                                    Ok(_) => {
                                                        println!("tag {} added.", next_version);
                                                        Ok(())
                                                    }
                                                    Err(e) => {
                                                        error!("git_add_tag error: {:?}", e);
                                                        Ok(())
                                                    }
                                                }
                                            }
                                            "minor" => {
                                                let tag = util::git_latest_tag()?;
                                                let version = Version::parse(&tag)?;
                                                let next_version = next_minor(version);
                                                match git_add_tag(&next_version.to_string()) {
                                                    Ok(_) => {
                                                        println!("tag {} added.", next_version);
                                                        Ok(())
                                                    }
                                                    Err(e) => {
                                                        error!("git_add_tag error: {:?}", e);
                                                        Ok(())
                                                    }
                                                }
                                            }
                                            "patch" => {
                                                let tag = util::git_latest_tag()?;
                                                let version = Version::parse(&tag)?;
                                                let next_version = next_patch(version);
                                                match git_add_tag(&next_version.to_string()) {
                                                    Ok(_) => {
                                                        println!("tag {} added.", next_version);
                                                        Ok(())
                                                    }
                                                    Err(e) => {
                                                        error!("git_add_tag error: {:?}", e);
                                                        Ok(())
                                                    }
                                                }
                                            }
                                            "phase" => {
                                                let tag = util::git_latest_tag()?;
                                                let version = Version::parse(&tag)?;
                                                let next_version = next_phase(version);
                                                match git_add_tag(&next_version.to_string()) {
                                                    Ok(_) => {
                                                        println!("tag {} added.", next_version);
                                                        Ok(())
                                                    }
                                                    Err(e) => {
                                                        error!("git_add_tag error: {:?}", e);
                                                        Ok(())
                                                    }
                                                }
                                            }
                                            "pre" => {
                                                let tag = util::git_latest_tag()?;
                                                let version = Version::parse(&tag)?;
                                                let next_version = next_pre(version);
                                                match git_add_tag(&next_version.to_string()) {
                                                    Ok(_) => {
                                                        println!("tag {} added.", next_version);
                                                        Ok(())
                                                    }
                                                    Err(e) => {
                                                        error!("git_add_tag error: {:?}", e);
                                                        Ok(())
                                                    }
                                                }
                                            }
                                            _ => {
                                                let _ = cmd.print_help();
                                                Ok(())
                                            }
                                        },
                                    }
                                }
                            }
                        }
                        "date" => {
                            let tag = util::git_latest_tag()?;
                            let date = util::git_date_of_hash(&tag)?;
                            print!("{}", date);
                            Ok(())
                        }
                        "hash" => {
                            let tag = util::git_latest_tag()?;
                            let hash = util::git_hash_of_tag(&tag)?;
                            print!("{}", hash);
                            Ok(())
                        }
                        "show" => {
                            let tag = util::git_latest_tag()?;
                            let tag_version = Version::parse(&tag)?;
                            let pre = next_pre(tag_version.clone()).to_string();
                            let phase = next_phase(tag_version.clone()).to_string();
                            let patch = next_patch(tag_version.clone()).to_string();
                            let minor = next_minor(tag_version.clone()).to_string();
                            let major = next_major(tag_version.clone()).to_string();
                            println!("Current tag: {}", tag);
                            println!("Next  phase: {}", phase);
                            println!("Next    pre: {}", pre);
                            println!("Next  patch: {}", patch);
                            println!("Next  minor: {}", minor);
                            println!("Next  major: {}", major);
                            Ok(())
                        }
                        _ => {
                            let _ = cmd.print_help();
                            Ok(())
                        }
                    },
                }
            }
            _ => {
                trace!("cmd_name: {:?}, args: {:?}", cmd_name, args);
                Ok(())
            }
        },
    }
}
