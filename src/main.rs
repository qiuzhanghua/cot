use clap::{command, Arg, ArgAction};

fn main() {
    let _matches = command!()
        .help_template(
            "{about}
{author}
        
Usage: {bin} [OPTIONS] [SUBCOMMAND]

Options:
{options}",
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
