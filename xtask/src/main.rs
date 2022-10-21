pub mod check;
pub mod test;

use std::path::{Path, PathBuf};

use clap::Parser;
use color_eyre::Help;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use xshell::{cmd, Shell};

static RUSTDOCFLAGS: &[&str] = &["--cfg", "nightly"];
static RUSTFLAGS: &[&str] = &["--cfg", "nightly"];
static TWITCH_API_DOC_FEATURES: &str =
    "twitch_oauth2/all twitch_oauth2/mock_api all unsupported deny_unknown_fields _all";
static FEATURE_SETS: &[&[&str]] = &[
    &["helix"],
    &["tmi"],
    &["helix", "tmi"],
    &["helix", "eventsub"],
    &["pubsub", "eventsub", "hmac"],
];
static OPT_FEATURES: &[&[&str]] = &[
    &["unsupported"],
    &["twitch_oauth2"],
    &["client", "trace_unknown_fields"],
    &["client", "unsupported"],
    &["deny_unknown_fields", "unsupported", "deser_borrow"],
    &["trace_unknown_fields", "deny_unknown_fields"],
];
static EXTRA_FEATURES: &[&[&str]] = &[&["pubsub", "twitch_types/time", "deny_unknown_fields"]];

#[derive(Debug, Parser)]
pub enum Args {
    Release,
    Doc {
        /// Set the target dir, this will by default be a subdirectory inside `target` to
        /// save on compilation, as the rust flags will be changed, thus needing a new compilation
        #[clap(long, default_value = "target/extra")]
        target_dir: String,
        #[clap(last = true)]
        last: Option<String>,
    },
    Check(check::Check),
    Test(test::Test),
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let sh = Shell::new()?;

    let args = Args::parse();

    match args {
        Args::Release => {
            let version = pkgid()?.rsplit_once('#').unwrap().1.to_string();
            color_eyre::eyre::ensure!(
                version.starts_with(|c: char| c.is_ascii_digit()),
                "version doesn't start with a number"
            );
            let tag = format!("v{version}");

            let has_tag = cmd!(sh, "git tag --list")
                .read()?
                .lines()
                .any(|it| it.trim() == tag);
            if !has_tag {
                let current_branch = cmd!(sh, "git branch --show-current").read()?;
                let default_branch = cmd!(
                    sh,
                    "gh repo view --json defaultBranchRef --jq .defaultBranchRef.name"
                )
                .read()?;
                let dry_run = sh.var("CI").is_err() || current_branch != default_branch;
                eprintln!("Taging!{}!", if dry_run { " (dry run)" } else { "" });

                let change_log =
                    std::fs::read_to_string(get_cargo_workspace().join("CHANGELOG.md"))?;

                if !tag.contains('-') {
                    color_eyre::eyre::ensure!(
                        change_log.contains(&format!("## [{tag}] -")),
                        "change log is not updated"
                    );
                }

                if dry_run {
                    eprintln!("{}", cmd!(sh, "git tag {tag}"));
                } else {
                    cmd!(sh, "git tag {tag}").run()?;
                }

                let dry_run_arg = if dry_run { Some("--dry-run") } else { None };
                cmd!(sh, "cargo publish {dry_run_arg...}").run()?;

                if dry_run {
                    eprintln!("{}", cmd!(sh, "git push origin {tag}"));
                } else {
                    cmd!(sh, "git push origin {tag}").run()?;
                }
            } else {
                eprintln!("tag exists already, no action needed");
            }
        }
        Args::Doc { target_dir, last } => {
            let target_dir = if std::env::var("CI").is_err() {
                vec!["--target-dir".to_owned(), target_dir]
            } else {
                vec![]
            };
            let last = last.as_deref();

            let target_dir = &target_dir;
            let _rustdocflags =
                sh.push_env("CARGO_ENCODED_RUSTDOCFLAGS", RUSTDOCFLAGS.join("\u{1f}"));
            let _rustflags = sh.push_env("CARGO_ENCODED_RUSTFLAGS", RUSTFLAGS.join("\u{1f}"));
            if !cargo_ver(&sh)?.contains("nightly") {
                color_eyre::eyre::bail!("Not running with a nightly cargo, use `cargo +nightly`");
            }

            let _section = section("Check");

            cmd!(
                sh,
                "cargo check {target_dir...} --features {TWITCH_API_FEATURES} --workspace"
            )
            .run()?;
            std::mem::drop(_section);
            let _section = section("First run");

            let res = cmd!(
                sh,
                "cargo doc {target_dir...} -v --no-deps --features {TWITCH_API_FEATURES} -Zunstable-options -Zrustdoc-scrape-examples=all -p twitch_api -p twitch_oauth2 -p twitch_types -Zrustdoc-map {last...}"
            )
            .run();
            std::mem::drop(_section);

            if std::env::var("CI").is_err() {
                res.with_suggestion(|| "try running again if rustdoc failed to load examples, see https://github.com/rust-lang/cargo/issues/10044")?;
            } else if res.is_err() {
                println!("::error title=doc with example scraping failed::couldn't document with scraped examples, using normal doc instead");
                cmd!(
                    sh,
                    "cargo doc {target_dir...} -v --no-deps --features {TWITCH_API_FEATURES} -p twitch_api -p twitch_oauth2 -p twitch_types -Zunstable-options -Zrustdoc-map {last...}"
                )
                .run()?;
            }
        }
        Args::Check(check) => check.run(&sh)?,
        Args::Test(test) => test.run(&sh)?,
    }
    Ok(())
}

fn cargo_ver(sh: &Shell) -> Result<String, color_eyre::Report> {
    cmd!(sh, "cargo -V").read().map_err(Into::into)
}

fn section(name: impl Into<String>) -> impl Drop {
    use std::io::Write;
    use std::time::Instant;
    let ci = std::env::var("CI").is_ok();
    let name = name.into();
    if ci {
        std::io::stdout().flush().unwrap();
        std::io::stderr().flush().unwrap();
        println!("::group::{name}");
    }
    let start = Instant::now();
    defer(move || {
        let elapsed = start.elapsed();
        eprintln!("{name}: {elapsed:.2?}");
        if ci {
            std::io::stdout().flush().unwrap();
            std::io::stderr().flush().unwrap();
            println!("::endgroup::");
        }
    })
}

fn defer<F: FnOnce()>(f: F) -> impl Drop {
    struct D<F: FnOnce()>(Option<F>);
    impl<F: FnOnce()> Drop for D<F> {
        fn drop(&mut self) {
            if let Some(f) = self.0.take() {
                f()
            }
        }
    }
    D(Some(f))
}

#[track_caller]
fn pkgid() -> Result<String, color_eyre::Report> {
    let sh = xshell::Shell::new()?;
    sh.change_dir(get_cargo_workspace());
    cmd!(sh, "cargo pkgid")
        .read()
        .map(|s| s.trim().to_owned())
        .map_err(Into::into)
}

/// Returns the cargo workspace for the manifest
pub fn get_cargo_workspace() -> &'static Path {
    static WORKSPACE: OnceCell<PathBuf> = OnceCell::new();
    #[derive(Debug, Deserialize)]
    pub struct CargoMetadata {
        pub workspace_root: PathBuf,
    }
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    WORKSPACE.get_or_init(|| {
        let sh = xshell::Shell::new().unwrap();
        sh.change_dir(manifest_dir);
        cmd!(sh, "cargo metadata --format-version 1 --no-deps")
            .read()
            .map_err(color_eyre::Report::from)
            .and_then(|s| serde_json::from_str::<CargoMetadata>(&s).map_err(Into::into))
            .unwrap()
            .workspace_root
    })
}

fn features() -> Vec<Vec<&'static str>> {
    let mut set: Vec<Vec<_>> = vec![all_features()];
    for feats in FEATURE_SETS.iter() {
        set.push(feats.iter().copied().sorted().collect());
        for opt in OPT_FEATURES.iter() {
            set.push(
                feats
                    .iter()
                    .copied()
                    .chain(opt.iter().copied())
                    .sorted()
                    .collect(),
            );
        }
    }
    // extras
    for extra in EXTRA_FEATURES {
        set.push(extra.to_vec())
    }
    set.sort_by_key(|f| -(f.len() as i32));
    set.insert(0, vec![]);
    set.into_iter().unique().collect()
}

fn all_features() -> Vec<&'static str> {
    crate::FEATURE_SETS
        .iter()
        .copied()
        .flatten()
        .chain(crate::OPT_FEATURES.iter().copied().flatten())
        .unique()
        .copied()
        .collect()
}

fn section(name: impl Into<String>) -> impl Drop {
    use std::time::Instant;
    let ci = std::env::var("CI").is_ok();
    let name = name.into();
    if ci {
        println!("::group::{name}");
    }
    let start = Instant::now();
    defer(move || {
        let elapsed = start.elapsed();
        eprintln!("{name}: {elapsed:.2?}");
        if ci {
            println!("::endgroup::");
        }
    })
}

fn defer<F: FnOnce()>(f: F) -> impl Drop {
    struct D<F: FnOnce()>(Option<F>);
    impl<F: FnOnce()> Drop for D<F> {
        fn drop(&mut self) {
            if let Some(f) = self.0.take() {
                f()
            }
        }
    }
    D(Some(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_pkgid_hashtag() {
        let pkgid = dbg!(pkgid().unwrap());
        assert!(!pkgid.contains('@'));
        assert!(pkgid.contains("twitch_api"));
    }
}
