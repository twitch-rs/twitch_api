use crate::{Opts, SubCommand};

#[derive(clap::Clap, Debug)]
pub struct Explore {
    #[clap(long)]
    pub path: Option<std::path::PathBuf>,
}

fn twitch_api2_path() -> String {
    let meta = cargo_metadata::MetadataCommand::new().exec().unwrap();
    let root = meta.root_package().expect("root not found");
    root.targets
        .iter()
        .find(|t| t.kind.iter().all(|k| k == "lib"))
        .unwrap()
        .src_path
        .to_string()
}

impl SubCommand for Explore {
    #[cfg(not(feature = "explore"))]
    fn run(&self, opts: &Opts) -> Result<(), anyhow::Error> {
        anyhow::bail!("`explore` feature not enabled")
    }

    #[cfg(feature = "explore")]
    fn run(&self, opts: &Opts) -> Result<(), anyhow::Error> { 
        let path = if let Some(p) = &self.path {
            p.clone()
        } else {
            twitch_api2_path().into()
        };
        anyhow::bail!("explore command is not implemented");
        Ok(()) }
}

#[cfg(feature = "explore")]
mod impls {
    pub use super::Explore;
    pub use crate::Opts;
}
