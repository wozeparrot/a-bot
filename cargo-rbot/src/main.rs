use clap::*;
use subprocess::*;

use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let matches = App::new("cargo-rbot")
        .about("Run it as cargo rbot <command>!")
        .version("0.1.0")
        .bin_name("cargo")
        .subcommand(
            SubCommand::with_name("rbot")
                .about("rbot command line tool for deploying and create rbot projects")
                .arg(
                    Arg::with_name("verbose")
                        .long("verbose")
                        .short("v")
                        .global(true)
                        .multiple(false)
                )
                .subcommand(
                    SubCommand::with_name("create")
                        .help("create a rbot project")
                        .arg(
                            Arg::with_name("NAME")
                                .required(true)
                                .index(1)
                                .help("name of project")
                        )
                        .arg(
                            Arg::with_name("TEAM")
                                .required(true)
                                .index(2)
                                .help("team number")
                        )
                )
                .setting(AppSettings::SubcommandRequiredElseHelp)
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    let rbot_matches = matches.subcommand_matches("rbot").expect("Failed");

    match rbot_matches.subcommand_name() {
        Some("create") => {
            create(rbot_matches.subcommand_matches("create").unwrap().value_of("NAME").expect("name not found"), rbot_matches.subcommand_matches("create").unwrap().value_of("TEAM").expect("name not found"))
        }
        Some("deploy") => {

        }
        _ => panic!("Unknown Subcommand")
    }
}

fn create(name: &str, team: &str) {
    fs::create_dir(name).expect("Directory Creation Failed");
    fs::create_dir(format!("{}/{}", name, "src/")).expect("Directory Creation Failed");

    let mut f = File::create(format!("{}/{}", name, "Cargo.toml")).expect("Cargo.toml Creation Failed");
    f.write_all(b"[package]\n").unwrap();
    f.write_all(b"name = ").unwrap();
    f.write_all(format!("\"{}\"", name).as_bytes()).unwrap();
    f.write_all(b"\nversion = \"0.1.0\"").unwrap();
    f.write_all(b"\nedition = \"2018\"").unwrap();
    f.write_all(b"\n").unwrap();
    f.write_all(b"\n[dependancies]\n").unwrap();
    f.write_all(b"rbotlib = \"0.0.2\"").unwrap();

    f.sync_all().unwrap();

    let mut f = File::create(format!("{}/{}", name, ".rbotconfig")).expect(".rbotconfig Creation Failed");
    f.write_all(b"[deploy]\n").unwrap();
    f.write_all(b"team = ").unwrap();
    f.write_all(format!("\"{}\"", team).as_bytes()).unwrap();

    f.sync_all().unwrap();
}