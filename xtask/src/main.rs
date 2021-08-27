use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = clap::crate_version!())]
struct Opts {
    #[clap(subcommand)]
    sub: SubCmd,
    #[clap(long)]
    feature: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Target {
    Lib,
    Doc,
    Example(String),
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Lib => f.write_str("lib"),
            Target::Doc => f.write_str("doc"),
            Target::Example(s) => write!(f, "example {}", s),
        }
    }
}

impl std::str::FromStr for Target {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "lib" => Target::Lib,
            "doc" => Target::Doc,
            example if example.starts_with("example ") => Target::Example(
                example
                    .split_once(' ')
                    .ok_or("No example mentioned")?
                    .1
                    .to_owned(),
            ),
            _ => return Err("No such target"),
        })
    }
}

#[derive(Clap, Debug)]
pub enum SubCmd {
    Clippy(ClippyCmd),
    Test(TestCmd),
}
#[derive(Clap, Debug)]
pub struct ClippyCmd {
    #[clap(long, default_values = &["helix", "tmi", "pubsub", "eventsub"])]
    endpoints: Vec<String>,
    #[clap(long, default_value = "unsupported deny_unknown_fields client")]
    features: String,
}

#[derive(Clap, Debug)]
pub struct TestCmd {
    #[clap(long, default_values = &["helix", "tmi", "pubsub", "eventsub"])]
    endpoints: Vec<String>,
    #[clap(long, default_value = "unsupported deny_unknown_fields client")]
    features: String,
    #[clap(long, default_values = &["lib", "doc"])]
    targets: Vec<Target>,
}

fn main() -> Result<(), anyhow::Error> {
    let opts: Opts = dbg!(Opts::try_parse()?);
    match opts.sub {
        SubCmd::Clippy(clippy) => {
            let endpoints = clippy.endpoints.join(" ");
            let features = clippy.features;
            xshell::cmd!("cargo clippy --all-targets --features {features} --features {endpoints}")
                .run()?;
        }

        SubCmd::Test(test) => {
            let endpoints = test.endpoints.join(" ");
            let features = test.features;
            xshell::cmd!("cargo test --features {features} --features {endpoints}")
                .run()?;
        }
    }
    Ok(())
}
