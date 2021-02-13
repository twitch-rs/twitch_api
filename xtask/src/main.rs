use clap::Clap;

#[derive(clap::Clap)]
#[clap(version = clap::crate_version!())]
struct Opts {

}

fn main() -> Result<(), anyhow::Error> {
    let _opts = Opts::try_parse()?;
    Ok(())
}
