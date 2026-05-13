use anyhow::{bail, Context, Result};
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("dist") => dist(),
        Some("ci") => ci(),
        Some("symlink-docs") => symlink_docs(),
        Some("check-env") => check_env(),
        Some(cmd) => {
            bail!("Unknown xtask command: {cmd}\n\nAvailable: dist, ci, symlink-docs, check-env")
        }
        None => {
            bail!("Usage: cargo xtask <command>\n\nAvailable: dist, ci, symlink-docs, check-env")
        }
    }
}

/// Build the release binary and copy it to bin/ for Git LFS tracking.
fn dist() -> Result<()> {
    println!("xtask dist: building release binary...");
    run("cargo", &["build", "--release", "--locked"])?;

    std::fs::create_dir_all("bin").context("create bin/")?;
    let src = "target/release/unifi";
    let dst = "bin/unifi";
    std::fs::copy(src, dst).with_context(|| format!("copy {src} → {dst}"))?;
    println!("xtask dist: copied {src} → {dst}");

    // Check LFS is installed
    let lfs_ok = Command::new("git")
        .args(["lfs", "version"])
        .output()
        .is_ok();
    if lfs_ok {
        println!("xtask dist: tracking bin/ via Git LFS...");
        run("git", &["lfs", "track", "bin/*"])?;
        run("git", &["add", ".gitattributes", dst])?;
        println!("xtask dist: staged {dst} for LFS commit");
    } else {
        eprintln!("WARNING: git-lfs not found — bin/unifi will not be LFS-tracked");
    }
    Ok(())
}

/// Run all CI checks: fmt, clippy, nextest, taplo, audit.
fn ci() -> Result<()> {
    println!("xtask ci: cargo fmt --check");
    run("cargo", &["fmt", "--", "--check"])?;

    println!("xtask ci: cargo clippy");
    run("cargo", &["clippy", "--", "-D", "warnings"])?;

    println!("xtask ci: cargo nextest run --profile ci");
    run("cargo", &["nextest", "run", "--profile", "ci"])?;

    println!("xtask ci: taplo check");
    run("taplo", &["check"])?;

    println!("xtask ci: cargo audit");
    run("cargo", &["audit"])?;

    println!("xtask ci: all checks passed");
    Ok(())
}

/// Symlink AGENTS.md and GEMINI.md to each CLAUDE.md found in the repo.
fn symlink_docs() -> Result<()> {
    let root = std::env::current_dir().context("get cwd")?;

    for entry in walkdir(&root)? {
        let path = entry?;
        if path.file_name() != Some(std::ffi::OsStr::new("CLAUDE.md")) {
            continue;
        }
        let dir = path.parent().unwrap_or(&root);
        for link_name in ["AGENTS.md", "GEMINI.md"] {
            let link_path = dir.join(link_name);
            // Remove stale symlink or wrong file
            if link_path.exists() || link_path.symlink_metadata().is_ok() {
                std::fs::remove_file(&link_path).ok();
            }
            #[cfg(unix)]
            std::os::unix::fs::symlink("CLAUDE.md", &link_path)
                .with_context(|| format!("symlink CLAUDE.md → {}", link_path.display()))?;
            #[cfg(not(unix))]
            eprintln!(
                "WARNING: symlinks not supported on this platform; skipping {}",
                link_path.display()
            );
            println!("symlink-docs: {} → CLAUDE.md", link_path.display());
        }
    }
    Ok(())
}

/// Validate that required environment variables are set.
fn check_env() -> Result<()> {
    let required = [
        (
            "UNIFI_URL",
            "UniFi controller base URL, e.g. https://unifi.local",
        ),
        (
            "UNIFI_API_KEY",
            "API key from UniFi OS → Settings → Admins & Users → API Keys",
        ),
    ];

    let mut missing = false;
    for (var, hint) in &required {
        match std::env::var(var) {
            Ok(v) if !v.is_empty() => println!("  ✓ {var}"),
            _ => {
                eprintln!("  ✗ {var} — {hint}");
                missing = true;
            }
        }
    }

    if missing {
        bail!(
            "Missing required environment variables — copy .env.example to .env and fill in values"
        );
    }
    println!("check-env: all required variables set");
    Ok(())
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn run(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to run: {program} {}", args.join(" ")))?;
    if !status.success() {
        bail!(
            "{program} {} failed with exit code {:?}",
            args.join(" "),
            status.code()
        );
    }
    Ok(())
}

/// Minimal recursive directory walker (avoids the walkdir dep).
fn walkdir(root: &std::path::Path) -> Result<impl Iterator<Item = Result<std::path::PathBuf>>> {
    let mut entries = Vec::new();
    collect_files(root, &mut entries)?;
    Ok(entries.into_iter().map(Ok))
}

fn collect_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).with_context(|| format!("read dir {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        // Skip hidden dirs, target/, node_modules/
        if name_str.starts_with('.') || name_str == "target" || name_str == "node_modules" {
            continue;
        }
        if path.is_dir() {
            collect_files(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}
