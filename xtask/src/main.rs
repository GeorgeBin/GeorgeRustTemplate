use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    RunDemo {
        #[arg(last = true)]
        args: Vec<OsString>,
    },
    BuildDemo,
    Check,
    Test,
    Lint,
    Verify,
    BuildRpm,
    BuildRpmLinux,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::RunDemo { args } => run_demo(args),
        Commands::BuildDemo => run(["cargo", "build", "-p", "demo", "--release"]),
        Commands::Check => run(["cargo", "check", "--workspace"]),
        Commands::Test => run(["cargo", "test", "--workspace"]),
        Commands::Lint => run([
            "cargo",
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ]),
        Commands::Verify => verify(),
        Commands::BuildRpm => build_rpm(),
        Commands::BuildRpmLinux => build_rpm_linux(),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn run_demo(args: Vec<OsString>) -> Result<(), String> {
    let mut command = Command::new("cargo");
    command.arg("run").arg("-p").arg("demo").arg("--");
    command.args(args);
    status(command)
}

fn verify() -> Result<(), String> {
    run(["cargo", "fmt", "--all", "--check"])?;
    run(["cargo", "check", "--workspace"])?;
    run(["cargo", "test", "--workspace"])?;
    run([
        "cargo",
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ])
}

fn build_rpm() -> Result<(), String> {
    run(["cargo", "build", "-p", "demo", "--release"])?;
    run(["cargo", "generate-rpm", "-p", "demo"])?;

    let version = read_demo_version()?;
    let src = find_first_rpm(Path::new("target/generate-rpm"))?;
    let dest = Path::new("build/x86_64-unknown-linux-gnu/release/rpm")
        .join(format!("demo-v{version}.rpm"));
    copy_artifact(&src, &dest)?;
    println!("{}", dest.display());
    Ok(())
}

fn build_rpm_linux() -> Result<(), String> {
    run(["rustup", "target", "add", "x86_64-unknown-linux-gnu"])?;
    let mut zigbuild = Command::new("cargo");
    zigbuild
        .env("RUST_FONTCONFIG_DLOPEN", "1")
        .arg("zigbuild")
        .arg("-p")
        .arg("demo")
        .arg("--release")
        .arg("--target")
        .arg("x86_64-unknown-linux-gnu");
    status(zigbuild)?;

    run([
        "cargo",
        "generate-rpm",
        "-p",
        "demo",
        "--target",
        "x86_64-unknown-linux-gnu",
        "--auto-req",
        "disabled",
        "--metadata-overwrite",
        "examples/rust-demo/packaging/linux/generate-rpm-cross.toml",
    ])?;

    let version = read_demo_version()?;
    let src = find_first_rpm(Path::new("target/x86_64-unknown-linux-gnu/generate-rpm"))?;
    let dest = Path::new("build/x86_64-unknown-linux-gnu/release/rpm")
        .join(format!("demo-v{version}.rpm"));
    copy_artifact(&src, &dest)?;
    println!("{}", dest.display());
    Ok(())
}

fn copy_artifact(src: &Path, dest: &Path) -> Result<(), String> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
        for entry in fs::read_dir(parent).map_err(|err| err.to_string())? {
            let path = entry.map_err(|err| err.to_string())?.path();
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("demo-v") && name.ends_with(".rpm"))
            {
                let _ = fs::remove_file(path);
            }
        }
    }

    fs::copy(src, dest).map_err(|err| err.to_string())?;
    Ok(())
}

fn read_demo_version() -> Result<String, String> {
    let manifest =
        fs::read_to_string("examples/rust-demo/Cargo.toml").map_err(|err| err.to_string())?;

    manifest
        .lines()
        .find_map(|line| line.strip_prefix("version = "))
        .map(|value| value.trim_matches('"').to_string())
        .ok_or_else(|| "failed to read demo version from examples/rust-demo/Cargo.toml".to_string())
}

fn find_first_rpm(dir: &Path) -> Result<PathBuf, String> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|err| err.to_string())?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "rpm"))
        .collect();
    entries.sort();
    entries
        .into_iter()
        .next()
        .ok_or_else(|| format!("no RPM artifacts found in {}", dir.display()))
}

fn run<const N: usize>(args: [&str; N]) -> Result<(), String> {
    let mut command = Command::new(args[0]);
    command.args(&args[1..]);
    status(command)
}

fn status(mut command: Command) -> Result<(), String> {
    let status = command.status().map_err(|err| err.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("command failed with status {status}"))
    }
}
