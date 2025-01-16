use clap::{command, Arg, ArgAction, Command};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "static/"]
#[prefix = "static/"]
struct Asset;

fn main() {
    for file in Asset::iter() {
        println!("{}", file.as_ref());
    }

    let _matches = command!()
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
        .subcommand(Command::new("xf").about("Extract <filename.tar.gz>"))
        .subcommand(Command::new("unzip").about("Extract <filename.zip>"))
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
        )
        .arg(
            Arg::new("verbose")
                .next_line_help(true)
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
}
