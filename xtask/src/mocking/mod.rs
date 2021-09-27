#[cfg(feature = "mocking")]
pub mod types;
#[cfg(feature = "mocking")]
pub mod mock;
#[cfg(feature = "mocking")]
pub mod mock_gen;
#[cfg(feature = "mocking")]
use impls::{MockingClient, Unit, UnitError, UnitResult, UnitType};
#[cfg(feature = "mocking")]
use types::*;

use crate::{Opts, SubCommand};

#[derive(clap::Clap, Debug)]
pub struct MockApi {
    // The location of the mock api server, eg. `http://localhost:8080`
    #[clap(long, default_value = "http://localhost:8080")]
    pub url: String,
    #[clap(long, takes_value = true)] // FIXME: global = true, blocked on https://github.com/clap-rs/clap/issues/1570 
    #[cfg(feature = "mocking")]
    pub unit: Option<UnitType>,
    #[clap(subcommand)]
    pub command: mock::MockAction,
}

impl SubCommand for MockApi {
    #[cfg(not(feature = "mocking"))]
    fn run(&self, opts: &Opts) -> Result<(), anyhow::Error> {
        anyhow::bail!("`mocking` feature not enabled")
    }

    #[cfg(feature = "mocking")]
    fn run(&self, opts: &Opts) -> Result<(), anyhow::Error> {
        let client = MockingClient::new(self, opts);

        // Only here for expanding if needed
        fn req<U: Unit>(client: &MockingClient) -> Result<UnitResult<U>, UnitError> {
            <U as Unit>::get(client)
        }

        match self.unit.as_ref().unwrap_or(&UnitType::NotSpecified) {
            UnitType::Category => {
                let units = req::<Category>(&client)?;
                client.evaluate(units)?;
            }
            UnitType::Client => {
                let units = req::<Client>(&client)?;
                client.evaluate(units)?;
            }
            UnitType::Stream => {
                let units = req::<Stream>(&client)?;
                client.evaluate(units)?;
            }
            UnitType::Subscription => {
                let units = req::<Subscription>(&client)?;
                client.evaluate(units)?;
            }
            UnitType::Tag => {
                let units = req::<Tag>(&client)?;
                client.evaluate(units)?;
            }
            UnitType::Team => {
                let units = req::<Team>(&client)?;
                client.evaluate(units)?;
            }
            UnitType::User => {
                let units = req::<User>(&client)?;
                client.evaluate(units)?;
            }
            UnitType::Video => {
                let units = req::<Video>(&client)?;
                client.evaluate(units)?;
            }
            UnitType::NotSpecified => todo!("not specified"),
            UnitType::Other(u) => todo!("unit `{}` not found", u),
        }
        Ok(())
    }
}

#[cfg(feature = "mocking")]
mod impls {
    use super::types::{Category, Client, Stream, Subscription, Tag, Team, User, Video};
    use super::mock;
    pub use super::MockApi;
    pub use crate::Opts;
    macro_rules! impl_unit {
        ($($name:ident => $path:literal),* $(,)?) => {
            $(
                impl Unit for $name {
                    const PATH: &'static str = $path;
                }
            )*
        };
    }

    impl_unit![
        Category => "categories",
        Client => "clients",
        Stream => "streams",
        Subscription => "subscriptions",
        Tag => "tags",
        Team => "teams",
        User => "users",
        Video => "videos",
    ];

    #[derive(serde::Deserialize, Debug)]
    #[serde(field_identifier, rename_all = "lowercase")]
    pub enum UnitType {
        #[serde(alias = "categories")]
        Category,
        #[serde(alias = "clients")]
        Client,
        #[serde(alias = "streams")]
        Stream,
        #[serde(alias = "subscriptions")]
        Subscription,
        #[serde(alias = "tags")]
        Tag,
        #[serde(alias = "teams")]
        Team,
        #[serde(alias = "users")]
        User,
        #[serde(alias = "videos")]
        Video,
        #[serde(skip)]
        NotSpecified,
        Other(String),
    }

    impl std::str::FromStr for UnitType {
        type Err = serde::de::value::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use serde::de::IntoDeserializer as _;
            use serde::Deserialize as _;
            Self::deserialize(s.into_deserializer())
        }
    }

    pub struct MockingClient<'o> {
        pub opts: &'o Opts,
        pub mock: &'o MockApi,
        pub client: ureq::Agent,
    }

    impl<'o> MockingClient<'o> {
        pub fn new(mock: &'o MockApi, opts: &'o Opts) -> Self {
            Self {
                client: ureq::AgentBuilder::new().build(),
                mock,
                opts,
            }
        }

        fn get_units<T: Unit>(&self) -> Result<String, ureq::Error> {
            let url = format!("{base}/units/{unit}", base = self.mock.url, unit = T::PATH);
            dbg!(&url);
            Ok(self.client.get(&url).call()?.into_string().unwrap())
        }
    }

    /// Units available in twitch-cli /unit
    pub trait Unit: std::fmt::Debug + Sized + serde::de::DeserializeOwned {
        /// Path to the unit, `{url}/units/{PATH}`
        const PATH: &'static str;

        fn deserialize_to_unit(source: &str) -> Result<UnitResult<Self>, serde_json::Error> {
            serde_json::from_str(source)
        }

        fn get(client: &MockingClient) -> Result<UnitResult<Self>, UnitError> {
            let response = client.get_units::<Self>().map_err(Box::new)?;

            Self::deserialize_to_unit(&response).map_err(Into::into)
        }
    }

    #[derive(thiserror::Error, Debug)]
    pub enum UnitError {
        #[error("could not deserialize unit result")]
        DeserError(#[from] serde_json::Error),
        #[error("could not request unit")]
        ClientError(#[from] Box<ureq::Error>),
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct UnitResult<T: Unit> {
        #[serde(bound = "T: Unit")]
        pub data: Vec<T>,
        pub cursor: Option<String>,
        pub total: i64,
    }
}
