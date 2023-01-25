//! Retrieves the list of available Cheermotes, animated emotes to which viewers can assign Bits, to cheer in chat.
//! [`get-cheermotes`](https://dev.twitch.tv/docs/api/reference#get-cheermotes)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetCheermotesRequest]
//!
//! To use this endpoint, construct a [`GetCheermotesRequest`] with the [`GetCheermotesRequest::new()`] or [`GetCheermotesRequest::builder()`] method.
//! If you do not provide an ID, the request will only include global cheermotes as defined by twitch.
//!
//! ```rust
//! use twitch_api::helix::bits::get_cheermotes;
//! let request = get_cheermotes::GetCheermotesRequest::broadcaster_id("1234");
//! // Without broadcaster ID
//! let request = get_cheermotes::GetCheermotesRequest::new();
//! ```
//!
//! ## Response: [Cheermote]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, bits::get_cheermotes};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_cheermotes::GetCheermotesRequest::new();
//! let response: Vec<get_cheermotes::Cheermote> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetCheermotesRequest::parse_response(None, &request.get_uri(), response)`](GetCheermotesRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Cheermotes](super::get_cheermotes)
///
/// [`get-cheermotes`](https://dev.twitch.tv/docs/api/reference#get-cheermotes)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetCheermotesRequest<'a> {
    /// ID for the broadcaster who might own specialized Cheermotes.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Option<Cow<'a, types::UserIdRef>>,
}

impl<'a> GetCheermotesRequest<'a> {
    /// Get available Cheermotes.
    pub fn new() -> Self { Self::default() }

    /// Get Cheermotes in a specific broadcasters channel.
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: Some(broadcaster_id.into_cow()),
        }
    }
}

/// Return Values for [Get Cheermotes](super::get_cheermotes)
///
/// [`get-cheermotes`](https://dev.twitch.tv/docs/api/reference#get-cheermotes)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
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
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
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
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
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
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CheermoteImages {
    /// Dark mode
    pub dark: CheermoteImage,
    /// Light mode
    pub light: CheermoteImage,
}

/// URLs to the same image, animated or static.
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CheermoteImage {
    /// Animated GIFs
    pub animated: CheermoteImageArray,
    /// Static PNGs
    #[serde(rename = "static")]
    pub static_: CheermoteImageArray,
}
/// Images in sizes
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
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

impl Request for GetCheermotesRequest<'_> {
    type Response = Vec<Cheermote>;

    const PATH: &'static str = "bits/cheermotes";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetCheermotesRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetCheermotesRequest::broadcaster_id("1234");

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

    dbg!(GetCheermotesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
