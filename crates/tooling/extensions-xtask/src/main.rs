use anyhow::{Context, Result, bail, ensure};
use clap::{Parser, Subcommand};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

mod causal_controls;
mod schema_controls;

const ESS_REPOSITORY: &str = "https://github.com/beyond10x/ess.git";
const ESS_REVISION: &str = "6ef4af76b99a8d2cd861a3cc76140c88c1361129";
const ESS_VERSION: &str = "ess 0.9.2";

#[derive(Parser)]
#[command(about = "Validate and generate the Extensions causal model draft")]
struct Cli {
    #[command(subcommand)]
    command: Task,
}

#[derive(Subcommand)]
enum Task {
    /// Validate sources, deterministic projections and contract refusal controls.
    Check,
    /// Regenerate the committed schema projections with the pinned compiler.
    Generate,
    /// Install the exact public ESS compiler used by the repository.
    InstallTools,
    /// Validate projected examples and numeric annotation boundaries.
    CheckSchema,
}

fn main() -> Result<()> {
    let task = Cli::parse().command;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .context("workspace root")?;
    let ess = compiler(root)?;
    match task {
        Task::InstallTools => println!("compiler: {ESS_VERSION} at {ESS_REVISION}"),
        Task::Generate => {
            let scratch = tempfile::tempdir()?;
            generate(&ess, &root.join("ess"), scratch.path())?;
            let contracts = root.join("contracts");
            // Replacing this tree is explicit only in the generate command. Check never edits it.
            if contracts.exists() {
                fs::remove_dir_all(&contracts)?;
            }
            copy_tree(scratch.path(), &contracts)?;
            println!("contracts regenerated");
        }
        Task::Check => check(root, &ess)?,
        Task::CheckSchema => schema_controls::check(root)?,
    }
    Ok(())
}

fn compiler(root: &Path) -> Result<PathBuf> {
    let tool_root = std::env::var_os("EXTENSIONS_ESS_TOOLCHAIN_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target/tools/ess"));
    let binary = tool_root.join("bin/ess");
    if !binary.is_file() || !pinned_installation(&tool_root) {
        successful(
            Command::new("cargo")
                .args([
                    "install",
                    "--locked",
                    "--force",
                    "--git",
                    ESS_REPOSITORY,
                    "--rev",
                    ESS_REVISION,
                    "--root",
                ])
                .arg(&tool_root)
                .arg("ess-cli"),
        )?;
    }
    ensure!(
        pinned_installation(&tool_root),
        "ESS installation source differs from {ESS_REVISION}"
    );
    let version = successful(Command::new(&binary).arg("--version"))?;
    ensure!(
        String::from_utf8(version.stdout)?.trim() == ESS_VERSION,
        "unexpected ESS version"
    );
    Ok(binary)
}

fn pinned_installation(root: &Path) -> bool {
    let Ok(bytes) = fs::read(root.join(".crates2.json")) else {
        return false;
    };
    let Ok(metadata) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return false;
    };
    metadata["installs"].as_object().is_some_and(|installs| {
        installs.keys().any(|key| {
            key.starts_with("ess-cli 0.9.2 (git+https://github.com/beyond10x/ess.git?")
                && key.ends_with(&format!("#{ESS_REVISION})"))
        })
    })
}

fn successful(command: &mut Command) -> Result<Output> {
    let output = command
        .output()
        .with_context(|| format!("start {command:?}"))?;
    ensure!(
        output.status.success(),
        "{command:?}\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(output)
}

fn generate(ess: &Path, source: &Path, out: &Path) -> Result<()> {
    successful(
        Command::new(ess)
            .args(["generate", "--path"])
            .arg(source)
            .args(["--kind", "schema", "--out"])
            .arg(out),
    )?;
    Ok(())
}

fn check(root: &Path, ess: &Path) -> Result<()> {
    successful(
        Command::new("cargo")
            .current_dir(root)
            .args(["fmt", "--all", "--check"]),
    )?;
    successful(Command::new("cargo").current_dir(root).args([
        "clippy",
        "--locked",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ]))?;
    let source = root.join("ess");
    let validation = successful(Command::new(ess).args(["validate", "--path"]).arg(&source))?;
    print!("{}", String::from_utf8(validation.stdout)?);
    let first = successful(
        Command::new(ess)
            .args(["compile", "--path"])
            .arg(&source)
            .args(["--format", "json"]),
    )?;
    let second = successful(
        Command::new(ess)
            .args(["compile", "--path"])
            .arg(&source)
            .args(["--format", "json"]),
    )?;
    ensure!(
        first.stdout == second.stdout,
        "canonical IR is not deterministic"
    );
    let ir: serde_json::Value = serde_json::from_slice(&first.stdout)?;
    ensure!(
        ir["entities"]
            .as_object()
            .is_some_and(|entities| entities.len() == 5),
        "the structural baseline must contain its five control records"
    );
    causal_controls::check(&ir)?;
    let scratch = tempfile::tempdir()?;
    let first_schema = scratch.path().join("first");
    let second_schema = scratch.path().join("second");
    generate(ess, &source, &first_schema)?;
    generate(ess, &source, &second_schema)?;
    let generated = tree(&first_schema)?;
    ensure!(
        !generated.is_empty(),
        "schema generation returned no artifacts"
    );
    ensure!(
        generated == tree(&second_schema)?,
        "schema generation is not deterministic"
    );
    ensure!(
        generated == tree(&root.join("contracts"))?,
        "contracts drift: run cargo xtask generate"
    );
    println!(
        "canonical IR and {} schema artifacts: deterministic and current",
        generated.len()
    );
    refusal_controls(ess, &source, scratch.path())?;
    schema_controls::check(root)?;
    println!("gate: valid causal model draft; operational scenarios remain unimplemented");
    Ok(())
}

fn refusal_controls(ess: &Path, source: &Path, scratch: &Path) -> Result<()> {
    let original = fs::read_to_string(source.join("domains/extensions.yaml"))?;
    let missing_target = original.replacen(
        "target: extensions.control.Release",
        "target: extensions.control.MissingRelease",
        1,
    );
    let wrong_carrier = original.replacen("via: extension_id", "via: specification", 1);
    let start = original
        .find("  - name: extensions.control.Extension\n")
        .context("Extension entity")?;
    let end = original[start..]
        .find("  - name: extensions.control.Release\n")
        .map(|offset| start + offset)
        .context("Release entity")?;
    let duplicate = original[start..end].replacen(
        "name: extensions.control.Extension\n",
        "name: extensions.control.SecondExtension\n",
        1,
    );
    let duplicate_owner = original.replacen(
        &original[start..end],
        &format!("{}{duplicate}", &original[start..end]),
        1,
    );
    let missing_cause = original.replacen(
        "moves: extensions.control.Installation.begin-activation",
        "updates: extensions.control.Installation",
        1,
    );
    let wrong_instance = original.replacen("instance: installation_id", "instance: attempt", 1);
    let mutating_refusal = original.replacen("wrong_state: true", "wrong_state: true\n        updates: extensions.control.Installation\n        instance: installation_id", 1);
    for (name, model, marker) in [
        ("unknown-target", missing_target, "MissingRelease"),
        ("wrong-carrier-type", wrong_carrier, "type_mismatch"),
        ("duplicate-owner", duplicate_owner, "owner"),
        ("missing-causation", missing_cause, "missing_causation"),
        ("wrong-instance-carrier", wrong_instance, "type_mismatch"),
        (
            "mutating-refusal",
            mutating_refusal,
            "refusal_mutated_state",
        ),
    ] {
        ensure!(
            model != original,
            "{name}: mutation did not change the model"
        );
        let fixture = scratch.join(name);
        copy_tree(source, &fixture)?;
        fs::write(fixture.join("domains/extensions.yaml"), model)?;
        let output = Command::new(ess)
            .args(["validate", "--path"])
            .arg(&fixture)
            .output()?;
        let diagnostic = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        ensure!(
            output.status.code() == Some(1),
            "{name}: expected semantic refusal, got {diagnostic}"
        );
        ensure!(
            diagnostic.to_lowercase().contains(&marker.to_lowercase()),
            "{name}: unrelated refusal: {diagnostic}"
        );
        println!("compiler refusal: {name}");
    }
    println!("compiler controls: 6 semantic refusals");
    Ok(())
}

fn tree(root: &Path) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
    fn visit(root: &Path, directory: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) -> Result<()> {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            let kind = entry.file_type()?;
            if kind.is_dir() {
                visit(root, &path, files)?;
            } else if kind.is_file() {
                files.insert(path.strip_prefix(root)?.to_owned(), fs::read(path)?);
            } else {
                bail!(
                    "unexpected non-regular projection entry: {}",
                    path.display()
                );
            }
        }
        Ok(())
    }
    let mut files = BTreeMap::new();
    visit(root, root, &mut files)?;
    Ok(files)
}

fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)?;
    for (path, bytes) in tree(source)? {
        let target = destination.join(path);
        fs::create_dir_all(target.parent().context("target parent")?)?;
        fs::write(target, bytes)?;
    }
    Ok(())
}
