mod metadata;
mod node;

use base64::{engine::general_purpose, Engine as _};
use cargo_metadata::{Message, MetadataCommand};
use clap::{Args, Parser, Subcommand};
use metadata::Metadata;
use node::transactions::*;
use sha256::digest;
use std::{
    env,
    fmt::Debug,
    fs,
    io::{Error, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const TARGET_WE: &str = "target/we";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    /// Toolkit for development WASM smart-contracts.
    #[clap(name = "we")]
    We(WeArgs),
}

#[derive(Args, Debug)]
struct WeArgs {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Initialize a new project.
    #[clap(name = "new")]
    New {
        /// The name of the newly created project.
        name: String,
        /// The optional target directory for the contract project.
        #[clap(short, long, value_parser)]
        target_dir: Option<PathBuf>,
    },
    /// Compiles the contract.
    #[clap(name = "build")]
    Build,
    /// Converts from the text format to the binary format.
    #[clap(name = "wat2wasm")]
    Wat2Wasm {
        filename: PathBuf,
        /// Output file for the generated wasm file.
        #[clap(short, long, value_parser)]
        output: Option<PathBuf>,
    },
    /// Converts from the binary format to the text format.
    #[clap(name = "wasm2wat")]
    Wasm2Wat {
        filename: PathBuf,
        /// Output file for the generated wat file, by default use stdout.
        #[clap(short, long, value_parser)]
        output: Option<PathBuf>,
    },
    /// Send tx by using Sign and Broadcast.
    #[clap(name = "tx")]
    Tx {
        /// Path to the transaction JSON configuration file
        path_json: PathBuf,
        /// Send the transaction flag
        #[clap(short, long, default_value_t = false)]
        send: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let Cli::We(args) = Cli::parse();

    match args.action {
        Action::New { name, target_dir } => new(name, target_dir),
        Action::Build => build(),
        Action::Wat2Wasm { filename, output } => wat2wasm(filename, output),
        Action::Wasm2Wat { filename, output } => wasm2wat(filename, output),
        Action::Tx { path_json, send } => tx(path_json, send).await,
    }
}

fn new(name: String, target_dir: Option<PathBuf>) -> Result<(), Error> {
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
        .open(format!(
            "{}/Cargo.toml",
            out_dir.to_str().expect("Failed to cast to string")
        ))?;

    write!(
        cargo_toml,
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
we-cdk = "0.4"
"#,
        name
    )?;

    let mut lib_rs = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!(
            "{}/lib.rs",
            out_dir.to_str().expect("Failed to cast to string")
        ))?;

    write!(
        lib_rs,
        r#"#![no_std]
#![no_main]
use we_cdk::*;

#[action]
fn _constructor(init_value: Boolean) {{
    set_storage!(boolean :: "value" => init_value);
}}

#[action]
fn flip() {{
    let value: Boolean = get_storage!(boolean :: "value");
    set_storage!(boolean :: "value" => !value);
}}
"#
    )?;

    let mut gitignore = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!(
            "{}/.gitignore",
            out_dir.to_str().expect("Failed to cast to string")
        ))?;

    write!(
        gitignore,
        r#"# Generated by Cargo
# will have compiled files and executables
debug/
target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# Used by macOS' file system to track custom attributes of containing folder
.DS_Store

# Editors' specific files
.idea/
.vscode/
"#
    )?;

    println!("Created contract {name}");

    Ok(())
}

fn build() -> Result<(), Error> {
    let metadata = MetadataCommand::new()
        .manifest_path("Cargo.toml")
        .exec()
        .expect("Unable to runs `cargo metadata`");

    let project_name = metadata
        .root_package()
        .expect("Unable to get root package")
        .name
        .as_str();

    fs::create_dir_all(TARGET_WE)?;

    let mut command = Command::new("cargo")
            .args([
                "+nightly",
                "build",
                "--release",
                "--message-format=json-render-diagnostics",
                "-Zbuild-std=std,panic_abort",
                "--target=wasm32-unknown-unknown",
                "--config=target.wasm32-unknown-unknown.rustflags = [\"-C\", \"target-feature=+bulk-memory,+multivalue\", \"-C\", \"link-args=--no-entry --import-memory -zstack-size=16 --initial-memory=131072 --max-memory=1048576\"]"
            ])
            .stdout(Stdio::piped())
            .spawn()?;

    let reader =
        std::io::BufReader::new(command.stdout.take().expect("Failed to get a read handle"));

    for message in cargo_metadata::Message::parse_stream(reader) {
        match message.expect("Unable to get message") {
            Message::CompilerArtifact(artifact) => {
                if artifact.target.name == project_name && !artifact.filenames.is_empty() {
                    if let Some(file_name) = artifact.filenames[0].file_name() {
                        fs::rename(
                            &artifact.filenames[0],
                            format!("{}/{}", TARGET_WE, file_name),
                        )?;
                    }
                }
            }
            Message::BuildFinished(finished) => {
                if finished.success {
                    let json = Metadata::new(project_name).as_json();

                    let mut metadata_file = fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(format!("{}/{}.json", TARGET_WE, project_name))?;

                    write!(metadata_file, "{}", json)?;
                }
            }
            _ => (),
        }
    }

    command.wait()?;

    Ok(())
}

fn wat2wasm(filename: PathBuf, output: Option<PathBuf>) -> Result<(), Error> {
    let output = match output {
        Some(path) => path
            .as_os_str()
            .to_str()
            .expect("Failed to cast to string")
            .to_string(),
        None => filename
            .clone()
            .file_name()
            .expect("")
            .to_str()
            .expect("Failed to cast to string")
            .replace(".wat", ".wasm"),
    };

    let binary = wat::parse_file(filename).expect("Failed to parse file");

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output)?;
    file.write_all(&binary)
}

fn wasm2wat(filename: PathBuf, output: Option<PathBuf>) -> Result<(), Error> {
    let wat = wasmprinter::print_file(filename).expect("");

    match output {
        Some(path) => {
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)?;

            file.write_all(wat.as_bytes())?;
        }
        None => println!("{}", wat),
    }

    Ok(())
}

async fn tx(path_json: PathBuf, send: bool) -> Result<(), Error> {
    let file = fs::read_to_string(path_json).expect("Can't read file");
    let mut config: Config = serde_json::from_str::<Config>(&file).expect("Can't parse json");

    let metadata = MetadataCommand::new()
        .manifest_path("Cargo.toml")
        .exec()
        .expect("Unable to runs `cargo metadata`");

    let project_name = metadata
        .root_package()
        .expect("Unable to get root package")
        .name
        .as_str();

    let path_wasm = format!("{}/{}.wasm", TARGET_WE, project_name);

    let bytecode = fs::read(path_wasm).expect("Can't read file");
    let bytecode_hash = digest(bytecode.clone());

    let stored_contract = StoredContractWasm {
        bytecode: general_purpose::STANDARD.encode(&bytecode),
        bytecode_hash,
    };
    config.transaction.set_stored_contract(stored_contract);

    if send {
        let node = node::Node::new(config.node_url, config.api_key);
        node.transaction_sign_and_broadcast(config.transaction)
            .await
            .expect("Failed to execute the request")
    } else {
        println!("Transaction before send:{}", config.transaction);
    }

    Ok(())
}
