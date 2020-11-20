//! Endpoints regarding bits
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, bits::GetCheermotesRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetCheermotesRequest::builder()
//!     .broadcaster_id("1234".to_string())
//!     .build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
#[doc(inline)]
pub use get_bits_leaderboard::{BitsLeaderboard, GetBitsLeaderboardRequest};
#[doc(inline)]
pub use get_cheermotes::{Cheermote, GetCheermotesRequest};

use crate::{helix, types};
use serde::{Deserialize, Serialize};

/// Retrieves the list of available Cheermotes, animated emotes to which viewers can assign Bits, to cheer in chat.
/// [`get-cheermotes`](https://dev.twitch.tv/docs/api/reference#get-cheermotes)
///
/// # Accessing the endpoint
///
/// ## Request: [GetCheermotesRequest]
///
/// To use this endpoint, construct a [`GetCheermotesRequest`] with the [`GetCheermotesRequest::builder()`] method.
/// If you do not provide an ID, the request will only include global cheermotes as defined by twitch.
///
/// ```rust, no_run
/// use twitch_api2::helix::bits::get_cheermotes;
/// let request = get_cheermotes::GetCheermotesRequest::builder()
///     .broadcaster_id(Some("1234".to_string()))
///     .build();
/// // Without broadcaster ID
/// let request = get_cheermotes::GetCheermotesRequest::builder().build();
/// ```
///
/// ## Response: [Cheermote]
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, bits::get_cheermotes};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
/// let request = get_cheermotes::GetCheermotesRequest::builder().build();
/// let response: Vec<get_cheermotes::Cheermote> = client.req_get(request, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
pub mod get_cheermotes {
    use super::*;

    /// Query Parameters for [Get Cheermotes](super::get_cheermotes)
    ///
    /// [`get-cheermotes`](https://dev.twitch.tv/docs/api/reference#get-cheermotes)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetCheermotesRequest {
        /// ID for the broadcaster who might own specialized Cheermotes.
        #[builder(default, setter(into))]
        pub broadcaster_id: Option<types::UserId>,
    }

    /// Return Values for [Get Cheermotes](super::get_cheermotes)
    ///
    /// [`get-cheermotes`](https://dev.twitch.tv/docs/api/reference#get-cheermotes)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct Cheermote {
        /// Indicates whether or not this emote provides a charity contribution match during charity campaigns.
        pub is_charitable: bool,
        /// The data when this Cheermote was last updated.
        pub last_updated: String,
        /// Order of the emotes as shown in the bits card, in ascending order.
        pub order: i64,
        /// Prefix for cheermote
        pub prefix: String,
        /// An array of Cheermotes with their metadata.
        pub tiers: Vec<Tiers>,
        /// Shows whether the emote is `global_first_party`,  `global_third_party`, `channel_custom`, `display_only`, or `sponsored`.
        #[serde(rename = "type")]
        pub type_: CheermoteType,
    }

    /// Types of cheermotes
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[serde(rename_all = "snake_case")]
    #[non_exhaustive]
    pub enum CheermoteType {
        /// Global cheermote
        GlobalFirstParty,
        /// Global third party cheermote
        GlobalThirdParty,
        /// Custom channel cheermote
        ChannelCustom,
        /// Display only
        ///
        /// # Note
        ///
        /// Usage unknown
        DisplayOnly,
        /// Sponsored cheermote
        Sponsored,
    }

    /// Information about tier of emote
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct Tiers {
        /// Indicates whether or not emote information is accessible to users.
        pub can_cheer: bool,
        /// Hex code for the color associated with the bits of that tier. Grey, Purple, Teal, Blue, or Red color to match the base bit type.
        pub color: String,
        /// ID of the emote tier. Possible tiers are: 1,100,500,1000,5000, 10k, or 100k.
        pub id: String,
        /// Structure containing both animated and static image sets, sorted by light and dark.
        pub images: CheermoteImages,
        /// Minimum number of bits needed to be used to hit the given tier of emote.  
        pub min_bits: i64,
        /// Indicates whether or not we hide the emote from the bits card.
        pub show_in_bits_card: bool,
    }

    /// Images for different themes
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct CheermoteImages {
        /// Dark mode
        pub dark: CheermoteImage,
        /// Light mode
        pub light: CheermoteImage,
    }

    /// URLs to the same image, animated or static.
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct CheermoteImage {
        /// Animated GIFs
        pub animated: CheermoteImageArray,
        /// Static PNGs
        #[serde(rename = "static")]
        pub static_: CheermoteImageArray,
    }
    /// Images in sizes
    #[derive(PartialEq, Eq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct CheermoteImageArray {
        /// URL to image in size 28 × 28
        #[serde(rename = "1")]
        pub url_1x: String,
        /// URL to image in size 42 × 42
        #[serde(rename = "1.5")]
        pub url_1_5x: String,
        /// URL to image in size 56 × 56
        #[serde(rename = "2")]
        pub url_2x: String,
        /// URL to image in size 84 × 84
        #[serde(rename = "3")]
        pub url_3x: String,
        /// URL to image in size 112 × 112
        #[serde(rename = "4")]
        pub url_4x: String,
    }

    /// Level of cheermote image
    ///
    /// Represented as "1", "1.5", "2", "3" or "4"
    #[derive(PartialEq, Eq, Hash, Deserialize, Debug, Clone)]
    #[serde(transparent)]
    pub struct Level(pub String);

    impl helix::Request for GetCheermotesRequest {
        type Response = Vec<Cheermote>;

        const PATH: &'static str = "bits/cheermotes";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetCheermotesRequest {}

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetCheermotesRequest::builder()
            .broadcaster_id("1234".to_string())
            .build();

        // From api call
        let data = br##"
{
    "data": [
        {
            "prefix": "Cheer",
            "tiers": [
                {
                    "min_bits": 1,
                    "id": "1",
                    "color": "#979797",
                    "images": {
                        "dark": {
                            "animated": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/1.gif",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/1.5.gif",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/2.gif",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/3.gif",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/4.gif"
                            },
                            "static": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/1.png",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/1.5.png",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/2.png",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/3.png",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/4.png"
                            }
                        },
                        "light": {
                            "animated": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/1.gif",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/1.5.gif",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/2.gif",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/3.gif",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/4.gif"
                            },
                            "static": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/1.png",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/1.5.png",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/2.png",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/3.png",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/4.png"
                            }
                        }
                    },
                    "can_cheer": true,
                    "show_in_bits_card": true
                },
                {
                    "min_bits": 100,
                    "id": "100",
                    "color": "#9c3ee8",
                    "images": {
                        "dark": {
                            "animated": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/100/1.gif",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/100/1.5.gif",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/100/2.gif",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/100/3.gif",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/100/4.gif"
                            },
                            "static": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/100/1.png",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/100/1.5.png",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/100/2.png",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/100/3.png",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/100/4.png"
                            }
                        },
                        "light": {
                            "animated": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/100/1.gif",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/100/1.5.gif",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/100/2.gif",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/100/3.gif",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/100/4.gif"
                            },
                            "static": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/100/1.png",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/100/1.5.png",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/100/2.png",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/100/3.png",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/100/4.png"
                            }
                        }
                    },
                    "can_cheer": true,
                    "show_in_bits_card": true
                },
                {
                    "min_bits": 1000,
                    "id": "1000",
                    "color": "#1db2a5",
                    "images": {
                        "dark": {
                            "animated": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1000/1.gif",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1000/1.5.gif",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1000/2.gif",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1000/3.gif",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1000/4.gif"
                            },
                            "static": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1000/1.png",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1000/1.5.png",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1000/2.png",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1000/3.png",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1000/4.png"
                            }
                        },
                        "light": {
                            "animated": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1000/1.gif",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1000/1.5.gif",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1000/2.gif",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1000/3.gif",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1000/4.gif"
                            },
                            "static": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1000/1.png",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1000/1.5.png",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1000/2.png",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1000/3.png",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1000/4.png"
                            }
                        }
                    },
                    "can_cheer": true,
                    "show_in_bits_card": true
                },
                {
                    "min_bits": 5000,
                    "id": "5000",
                    "color": "#0099fe",
                    "images": {
                        "dark": {
                            "animated": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/5000/1.gif",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/5000/1.5.gif",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/5000/2.gif",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/5000/3.gif",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/5000/4.gif"
                            },
                            "static": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/5000/1.png",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/5000/1.5.png",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/5000/2.png",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/5000/3.png",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/5000/4.png"
                            }
                        },
                        "light": {
                            "animated": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/5000/1.gif",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/5000/1.5.gif",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/5000/2.gif",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/5000/3.gif",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/5000/4.gif"
                            },
                            "static": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/5000/1.png",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/5000/1.5.png",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/5000/2.png",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/5000/3.png",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/5000/4.png"
                            }
                        }
                    },
                    "can_cheer": true,
                    "show_in_bits_card": true
                },
                {
                    "min_bits": 10000,
                    "id": "10000",
                    "color": "#f43021",
                    "images": {
                        "dark": {
                            "animated": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/10000/1.gif",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/10000/1.5.gif",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/10000/2.gif",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/10000/3.gif",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/10000/4.gif"
                            },
                            "static": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/10000/1.png",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/10000/1.5.png",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/10000/2.png",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/10000/3.png",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/10000/4.png"
                            }
                        },
                        "light": {
                            "animated": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/10000/1.gif",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/10000/1.5.gif",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/10000/2.gif",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/10000/3.gif",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/10000/4.gif"
                            },
                            "static": {
                                "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/10000/1.png",
                                "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/10000/1.5.png",
                                "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/10000/2.png",
                                "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/10000/3.png",
                                "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/10000/4.png"
                            }
                        }
                    },
                    "can_cheer": true,
                    "show_in_bits_card": true
                }
            ],
            "type": "global_first_party",
            "order": 1,
            "last_updated": "2018-05-22T00:06:04Z",
            "is_charitable": false
        }
    ]
}
"##
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/bits/cheermotes?broadcaster_id=1234"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Gets a ranked list of Bits leaderboard information for an authorized broadcaster.
/// [`get-bits-leaderboard`](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard)
///
/// # Accessing the endpoint
///
/// ## Request: [GetBitsLeaderboardRequest]
///
/// To use this endpoint, construct a [`GetBitsLeaderboardRequest`] with the [`GetBitsLeaderboardRequest::builder()`] method.
/// Provide [`started_at`](GetBitsLeaderboardRequest::started_at) and [`period`](GetBitsLeaderboardRequest::period) to get a different leaderboard than default
///
///
/// ```rust, no_run
/// use twitch_api2::helix::bits::get_bits_leaderboard;
/// let request = get_bits_leaderboard::GetBitsLeaderboardRequest::builder()
///     .started_at("2020-01-01T07:00:00Z".to_string())
///     .period("day".to_string())
///     .build();
/// // Get leaderbord for the lifetime of the channel
/// let request = get_bits_leaderboard::GetBitsLeaderboardRequest::builder().build();
/// ```
///
/// ## Response: [BitsLeaderboard]
///
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, bits::get_bits_leaderboard};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
/// let request = get_bits_leaderboard::GetBitsLeaderboardRequest::builder().build();
/// let response: get_bits_leaderboard::BitsLeaderboard = client.req_get(request, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
pub mod get_bits_leaderboard {
    use std::convert::TryInto;

    use super::*;

    /// Query Parameters for [Get Bits Leaderboard](super::get_bits_leaderboard)
    ///
    /// [`get-bits-leaderboard`](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetBitsLeaderboardRequest {
        /// Number of results to be returned. Maximum: 100. Default: 10.
        #[builder(default, setter(into))]
        pub count: Option<i32>,
        // TODO: Enum
        /// Time period over which data is aggregated (PST time zone). This parameter interacts with started_at. Valid values follow. Default: "all".
        ///
        /// * "day" – 00:00:00 on the day specified in started_at, through 00:00:00 on the following day.
        /// * "week" – 00:00:00 on Monday of the week specified in started_at, through 00:00:00 on the following Monday.
        /// * "month" – 00:00:00 on the first day of the month specified in started_at, through 00:00:00 on the first day of the following month.
        /// * "year" – 00:00:00 on the first day of the year specified in started_at, through 00:00:00 on the first day of the following year.
        /// * "all" – The lifetime of the broadcaster's channel. If this is specified (or used by default), started_at is ignored.
        #[builder(default, setter(into))]
        pub period: Option<String>,
        /// Timestamp for the period over which the returned data is aggregated. Must be in RFC 3339 format. If this is not provided, data is aggregated over the current period; e.g., the current day/week/month/year. This value is ignored if period is "all".
        #[builder(default, setter(into))]
        pub started_at: Option<types::Timestamp>,
        /// ID of the user whose results are returned; i.e., the person who paid for the Bits.
        #[builder(default, setter(into))]
        pub user_id: Option<types::UserId>,
    }

    /// Return Values for [Get Bits Leaderboard](super::get_bits_leaderboard)
    ///
    /// [`get-bits-leaderboard`](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct BitsLeaderboard {
        /// Leaderboard
        pub leaderboard: Vec<LeaderboardUser>,
        /// Period over which the returned data is aggregated.
        pub date_range: DateRange,
        /// Total number of results (users) returned. This is count or the total number of entries in the leaderboard, whichever is less.
        pub total: i64,
    }

    /// Period over which the returned data is aggregated.
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct DateRange {
        /// Start of the date range for the returned data.
        pub started_at: types::Timestamp,
        /// End of the date range for the returned data.
        pub ended_at: types::Timestamp,
    }

    /// Information about user in leaderboard
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct LeaderboardUser {
        /// Leaderboard rank of the user.
        pub rank: i64,
        /// Leaderboard score (number of Bits) of the user
        pub score: i64,
        /// ID of the user (viewer) in the leaderboard entry.
        pub user_id: types::UserId,
        /// Display name corresponding to user_id.
        pub user_name: types::UserName,
    }

    impl helix::Request for GetBitsLeaderboardRequest {
        type Response = BitsLeaderboard;

        const PATH: &'static str = "bits/leaderboard";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetBitsLeaderboardRequest {
        fn parse_response(
            self,
            uri: &http::Uri,
            response: http::Response<Vec<u8>>,
        ) -> Result<
            helix::Response<Self, <Self as helix::Request>::Response>,
            helix::HelixRequestGetError,
        >
        where
            Self: Sized,
        {
            #[derive(PartialEq, Deserialize, Debug, Clone)]
            struct InnerResponse {
                data: Vec<LeaderboardUser>,
                date_range: DateRange,
                /// Total number of results (users) returned. This is count or the total number of entries in the leaderboard, whichever is less.
                total: i64,
            }

            let text = std::str::from_utf8(&response.body())
                .map_err(|e| helix::HelixRequestGetError::Utf8Error(response.body().clone(), e))?;
            //eprintln!("\n\nmessage is ------------ {} ------------", text);
            if let Ok(helix::HelixRequestError {
                error,
                status,
                message,
            }) = serde_json::from_str::<helix::HelixRequestError>(&text)
            {
                return Err(helix::HelixRequestGetError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                });
            }
            let response: InnerResponse = serde_json::from_str(&text)?;
            Ok(helix::Response {
                data: BitsLeaderboard {
                    leaderboard: response.data,
                    date_range: response.date_range,
                    total: response.total,
                },
                pagination: None,
                request: self,
            })
        }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetBitsLeaderboardRequest::builder().build();

        // From api call
        let data = br##"
{
    "data": [
        {
        "user_id": "1234",
        "user_name": "tmi",
        "rank": 1,
        "score": 1234567
        }
    ],
    "date_range": {
        "started_at": "2020-01-01T07:00:00Z",
        "ended_at": "2021-01-01T07:00:00Z"
    },
    "total": 10
    }
"##
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/bits/leaderboard?"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}
