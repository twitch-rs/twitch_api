use super::{Unit, UnitResult};
#[derive(Debug, clap::Clap)]
pub enum MockAction {
    #[clap(name = "random")]
    Random(Random),
    #[clap(name = "run")]
    RunMock(Example),
}

pub fn get_random<T: Unit>(source: &UnitResult<T>) -> Option<&T> {
    let i = fastrand::usize(..source.data.len());
    source.data.get(i)
}

impl super::MockingClient<'_> {
    pub fn evaluate<U: Unit>(&self, units: UnitResult<U>) -> Result<(), anyhow::Error> {
        match &self.mock.command {
            MockAction::Random(ref random) => {
                if let Some(rand) = get_random(&units) {
                    println!("{:?}", rand);
                }
            }
            MockAction::RunMock(e) => e.run(self)?,
        }
        Ok(())
    }
}

#[derive(Debug, clap::Clap)]
pub struct Random {}

#[derive(Debug, clap::Clap)]
pub struct Example {}

impl Example {
    pub fn run(&self, client: &'_ super::MockingClient) -> Result<(), anyhow::Error> {
        //xshell::cmd!("cargo run -p ")
        todo!()
    }
}