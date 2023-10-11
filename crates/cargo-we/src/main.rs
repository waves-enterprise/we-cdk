use cargo_metadata::Message;
use clap::{Args, Parser, Subcommand};
use std::{
    env, fs,
    io::{Error, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    #[clap(name = "we")]
    We(WeArgs),
}

#[derive(Args, Debug)]
struct WeArgs {
    #[clap(subcommand)]
    cmd: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    #[clap(name = "new")]
    New {
        name: String,
        #[clap(short, long, value_parser)]
        target_dir: Option<PathBuf>,
    },
    #[clap(name = "build")]
    Build,
}

fn main() -> Result<(), Error> {
    let Cli::We(args) = Cli::parse();

    match args.cmd {
        Action::New { name, target_dir } => {
            let out_dir = target_dir
                .map_or(env::current_dir()?, |p| {
                    <PathBuf as AsRef<Path>>::as_ref(&p).to_path_buf()
                })
                .join(name.clone());
            if out_dir.join("Cargo.toml").exists() {
                println!("A Cargo package already exists in {}", name);
            }
            if !out_dir.exists() {
                fs::create_dir(&out_dir)?;
            }

            let mut cargo_toml = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(format!("{}/Cargo.toml", out_dir.to_str().unwrap()))?;

            write!(
                cargo_toml,
                "{}",
                format!(
                    r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib"]
path = "lib.rs"

[profile.release]
codegen-units = 1
lto = true
opt-level = 'z'
panic = 'abort'
strip = true

[dependencies]
we-contract-sdk = {{ version = "0.1.0", path = "../crates/sdk" }}

            "#,
                    name
                )
            )?;

            let mut lib_rs = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(format!("{}/lib.rs", out_dir.to_str().unwrap()))?;

            write!(
                lib_rs,
                "{}",
                format!(
                    r#"use we_contract_sdk::*;

#[action]
fn _constructor(init_value: Boolean) {{
    set_storage!(boolean => ("value", init_value));
}}

#[action]
fn flip() {{
    let value: Boolean = get_storage!(boolean "value");
    set_storage!(boolean => ("value", !value));
}}

            "#
                )
            )?;

            println!("Created contract {name}");

            Ok(())
        }
        Action::Build => {
            let mut command = Command::new("cargo")
            .args(&[
                "+nightly",
                "build",
                "--release",
                "-Zbuild-std=std,panic_abort",
                "--target=wasm32-unknown-unknown",
                "--config=target.wasm32-unknown-unknown.rustflags = [\"-C\", \"target-feature=+bulk-memory,+multivalue\", \"-C\", \"link-args=--no-entry --import-memory -zstack-size=16 --initial-memory=131072 --max-memory=1048576\"]"
            ])
            .stdout(Stdio::piped())
            .spawn()?;

            let reader = std::io::BufReader::new(command.stdout.take().unwrap());

            for message in cargo_metadata::Message::parse_stream(reader) {
                match message.unwrap() {
                    Message::CompilerMessage(msg) => {
                        println!("{:?}", msg);
                    }
                    Message::CompilerArtifact(artifact) => {
                        println!("{:?}", artifact);
                    }
                    Message::BuildScriptExecuted(script) => {
                        println!("{:?}", script);
                    }
                    Message::BuildFinished(finished) => {
                        println!("{:?}", finished);
                    }
                    _ => (),
                }
            }

            command.wait()?;

            Ok(())
        }
    }
}
