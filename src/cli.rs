use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn new() -> ArgMatches {
    let app = Command::new("Diglish")
        .version("1.0")
        .author("Ton <my_eleanor.ton@pm.me")
        .about("Cambridge dictionary scrapper")
        .arg(Arg::new("word"))
        .arg(
            Arg::new("clip")
                .short('c')
                .long("clipboard")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("notify")
                .short('n')
                .long("notify")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    return app;
}
