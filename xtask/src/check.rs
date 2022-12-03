use itertools::Itertools;
use xshell::{cmd, Shell};

#[derive(Debug, clap::Args)]
pub struct Check {
    #[clap(long)]
    list: bool,
    #[clap(long)]
    locked: bool,
    #[clap(last = true)]
    rest: Vec<String>,
}

impl Check {
    pub fn run(&self, sh: &Shell) -> Result<(), color_eyre::Report> {
        let Check { rest, locked, list } = self;
        let set = crate::features();
        if *list {
            println!("{}", set.iter().map(|v| v.join(", ")).join("\n"));
            return Ok(());
        }
        println!("Doing {} checks", set.len());
        let locked = if *locked { Some("--locked") } else { None };
        for features in set {
            let features = if !features.is_empty() {
                Some(format!("--no-default-features --features={}", features.iter().join(",")))
            } else {
                None
            };
            let _section = crate::section(format!(
                "Features: {}",
                features.as_deref().unwrap_or_default()
            ));
            cmd!(
                sh,
                "cargo clippy {locked...} --all-targets {features...} -- {rest...}"
            )
            .run()?;
        }
        Ok(())
    }
}
