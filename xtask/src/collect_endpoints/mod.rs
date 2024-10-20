use std::path::Path;

use color_eyre::eyre::{Context, OptionExt, Result};
use std::fs;
use url::Url;
use xshell::{cmd, Shell};

mod eventsub;
mod helix;
pub mod html;
pub mod rustdoc;

pub static HELIX_SOURCE_FILE: &str = "src/helix/mod.rs";
static HELIX_URL: &str = "https://dev.twitch.tv/docs/api/reference";

pub static EVENTSUB_SOURCE_FILE: &str = "src/eventsub/mod.rs";
static EVENTSUB_URL: &str = "https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types";

pub fn run(sh: &Shell, build_json_docs: bool, all_features: &str) -> Result<()> {
    let helix_h = std::thread::spawn(|| -> Result<String> {
        ureq::get(HELIX_URL)
            .call()?
            .into_string()
            .map_err(Into::into)
    });
    let eventsub_h = std::thread::spawn(|| -> Result<String> {
        ureq::get(EVENTSUB_URL)
            .call()?
            .into_string()
            .map_err(Into::into)
    });

    if build_json_docs {
        let _section = crate::section("cargo doc");
        cmd!(
            sh,
            "cargo doc --no-deps --features {all_features} -p twitch_api"
        )
        .env("RUSTDOCFLAGS", "-Zunstable-options --output-format json")
        .run()?;
    }
    let json = sh.read_file("target/doc/twitch_api.json")?;
    let rustdoc = serde_json::from_str(&json)?;
    let (mut helix_rustdoc, eventsub_rustdoc) = rustdoc::parse(&rustdoc)?;

    let (helix, eventsub) = (
        helix_h.join().expect("failed to join helix thread")?,
        eventsub_h.join().expect("failed to join eventsub thread")?,
    );

    std::thread::scope(|s| -> Result<()> {
        let h1 = s.spawn(|| {
            let overview =
                helix::make_overview(&Url::parse(HELIX_URL).unwrap(), &helix, &mut helix_rustdoc)?;
            paste_in_file(HELIX_SOURCE_FILE, overview)
        });
        let h2 = s.spawn(|| {
            let overview = eventsub::make_overview(
                &Url::parse(EVENTSUB_URL).unwrap(),
                &eventsub,
                &eventsub_rustdoc,
            )?;
            paste_in_file(EVENTSUB_SOURCE_FILE, overview)
        });

        h1.join().expect("failed to join helix-write thread")?;
        h2.join().expect("failed to join eventsub-write thread")?;
        Ok(())
    })
}

pub fn indicator_for(actual: usize, max: usize) -> char {
    if actual == 0 {
        '🔴'
    } else if actual == max {
        '🟢'
    } else {
        '🟡'
    }
}

fn paste_in_file(path: impl AsRef<Path>, to_paste: String) -> Result<()> {
    let path = path.as_ref();

    let begin_marker = "//! <!-- BEGIN-OVERVIEW -->";
    let end_marker = "//! <!-- END-OVERVIEW -->";

    let content = fs::read_to_string(path).with_context(|| format!("opening {path:?}"))?;

    let (before, inside_and_after) = content
        .split_once(begin_marker)
        .ok_or_eyre("No begin marker in file")?;
    let (_inside, after) = inside_and_after
        .split_once(end_marker)
        .ok_or_eyre("no end marker")?;

    fs::write(
        path,
        format!("{before}{begin_marker}\n{to_paste}{end_marker}{after}"),
    )?;

    Ok(())
}
