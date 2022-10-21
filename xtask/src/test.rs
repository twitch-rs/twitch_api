use itertools::Itertools;
use xshell::{cmd, Shell};

#[derive(Debug, clap::Args)]
pub struct Test {
    #[clap(long)]
    no_nextest: bool,
    #[clap(last = true)]
    rest: Vec<String>,
}

impl Test {
    pub fn run(&self, sh: &Shell) -> Result<(), color_eyre::Report> {
        let Test { rest, no_nextest } = self;
        let test: &[_] = if *no_nextest {
            &["test"]
        } else {
            &["nextest", "run"]
        };

        let set = crate::features();
        println!("Doing {} test runs", set.len());
        for features in set {
            let features = features.join(",");
            let _section = crate::section(format!("Features: {features}"));
            cmd!(sh, "cargo {test...} --features {features} -- {rest...}").run()?;
        }

        Ok(())
    }
}
