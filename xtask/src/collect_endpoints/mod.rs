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
pub static HELIX_ENDPOINTS_FOLDER: &str = "src/helix/endpoints/";
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
        let h1 = s.spawn(|| -> Result<()> {
            let overview =
                helix::make_overview(&Url::parse(HELIX_URL).unwrap(), &helix, &mut helix_rustdoc)?;
            paste_in_file(HELIX_SOURCE_FILE, overview.0)?;
            for (module, doc) in overview.1 {
                let path = format!("{}{}/mod.rs", HELIX_ENDPOINTS_FOLDER, module);
                if std::fs::exists(path.as_str())? {
                    paste_in_file(path, doc)?;
                }
            }
            Ok(())
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

pub const fn indicator_for(actual: usize, max: usize) -> char {
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

pub fn levenshtein(src: &str, tar: &str) -> usize {
    let src_len = src.chars().count();
    let tar_len = tar.chars().count();

    // initialize the matrix
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; tar_len + 1]; src_len + 1];

    for i in 1..(src_len + 1) {
        matrix[i][0] = i;
    }

    for i in 1..(tar_len + 1) {
        matrix[0][i] = i;
    }

    // apply edit operations
    for (i, s_char) in src.chars().enumerate() {
        for (j, t_char) in tar.chars().enumerate() {
            let substitution_cost = if s_char == t_char { 0 } else { 1 };
            let operations = [
                matrix[i][j + 1] + 1,             // deletion
                matrix[i + 1][j] + 1,             // insertion
                matrix[i][j] + substitution_cost, // substitution
            ];
            matrix[i + 1][j + 1] = *operations.iter().min().unwrap();
        }
    }

    matrix[src_len][tar_len]
}
