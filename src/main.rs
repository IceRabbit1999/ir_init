use std::{fs, fs::File, io::Write, path::Path};

use anyhow::bail;
use clap::Parser;

use crate::args::{Args, Commands};

mod args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        None => {}
        Some(command) => match command {
            Commands::Init { name } => {
                if let Err(e) = default_init(&name) {
                    eprintln!("Error: {}", e);
                }
            }
        },
    }
    Ok(())
}

/// Initialize a new project with the given name using `Cargo new <name>` with a workspace-style
/// layout.
fn default_init(name: &str) -> anyhow::Result<()> {
    // Cargo new <name>
    let status = std::process::Command::new("cargo")
        .arg("new")
        .arg(name)
        .status()?;
    if !status.success() {
        bail!("Failed to create new project with `cargo new`");
    }

    // Change directory to the new project
    let project_path = Path::new(name);

    // Rewrite `Cargo.toml` to be a workspace style
    let workspace = r#"[workspace]
members = ["crates/*"]
resolver = "2"
"#;
    let cargo_toml_path = project_path.join("Cargo.toml");
    let mut file = File::create(&cargo_toml_path)?;
    file.write_all(workspace.as_bytes())?;

    // Add `rustfmt.toml` to the root of the project
    let fmt = r#"imports_granularity="Crate"
wrap_comments=true
comment_width=100
group_imports="StdExternalCrate"
"#;
    let mut file = File::create(project_path.join("rustfmt.toml"))?;
    file.write_all(fmt.as_bytes())?;

    // Create `crates` directory and new binary crate inside it
    let crates_path = project_path.join("crates");
    fs::create_dir(&crates_path)?;
    let status = std::process::Command::new("cargo")
        .arg("new")
        .arg("--bin")
        .arg("app")
        .current_dir(&crates_path)
        .status()?;
    if !status.success() {
        bail!("Failed to create new project in `crates` directory");
    }

    // Add default dependencies
    let status = std::process::Command::new("cargo")
        .arg("add")
        .arg("snafu")
        .arg("tracing")
        .arg("tracing-subscriber")
        .arg("ir_aquila")
        .current_dir(project_path)
        .status()?;
    if !status.success() {
        bail!("Failed to add default dependencies");
    }

    Ok(())
}
