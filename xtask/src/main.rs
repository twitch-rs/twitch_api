use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = clap::crate_version!())]
struct Opts {
    #[clap(subcommand)]
    sub: SubCmd,
    #[clap(long, default_value = "twitch_oauth2/all unsupported deny_unknown_fields client")]
    features: String,
    #[clap(long, default_values = &["lib","doc","examples"])]
    targets: Vec<Target>,
    #[clap(long)]
    feature: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Target {
    Lib,
    Doc,
    Examples,
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Lib => f.write_str("lib"),
            Target::Doc => f.write_str("doc"),
            Target::Examples => f.write_str("examples"),
        }
    }
}

impl std::str::FromStr for Target {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "lib" => Target::Lib,
            "doc" => Target::Doc,
            "examples" => Target::Examples,
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
}

#[derive(Clap, Debug)]
pub struct TestCmd {
    #[clap(long, default_values = &["helix", "tmi", "pubsub", "eventsub"])]
    endpoints: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let opts: Opts = dbg!(Opts::try_parse()?);
    let features = opts.features;
    match opts.sub {
        SubCmd::Clippy(clippy) => {
            let endpoints = clippy.endpoints.join(" ");
            for target in opts.targets.iter().filter(|t|  !&[Target::Doc].iter().any(|o| t == &o )) {
                let target = target.to_string();
                xshell::cmd!("cargo clippy --{target} --features {features} --features {endpoints}").run()?;
            }
        }

        SubCmd::Test(test) => {
            let endpoints = test.endpoints.join(" ");
            for target in opts.targets {
                let target = target.to_string();
                xshell::cmd!("cargo test --{target} --features {features} --features {endpoints}").run()?;
            }
        }
    }
    Ok(())
}
