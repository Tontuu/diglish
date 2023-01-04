use anyhow::{Context, Result};
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
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    return app;
}

pub fn clipboard(meaning: String) -> Result<(), Box<dyn std::error::Error>> {
    let command = format!("echo -n '{}'", meaning);

    // Spawn the sh/echo process
    let echo = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", command.as_str()])
            .stdout(std::process::Stdio::piped())
            .spawn()
    } else {
        std::process::Command::new("sh")
            .args(&["-c", command.as_str()])
            .stdout(std::process::Stdio::piped())
            .spawn()
    }
    .with_context(|| format!("Cannot find shell binary path"))?;

    let _xclip = std::process::Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(std::process::Stdio::from(
            echo.stdout
                .expect("Failed to open echo stdout in xclip process"),
        ))
        .stdout(std::process::Stdio::piped())
        .spawn()
        .with_context(|| format!("Cannot find xclip binary path"))?;

    Ok(())
}
