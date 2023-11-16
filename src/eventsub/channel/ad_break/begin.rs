#![doc(alias = "channel.ad_break.begin")]
//! a user runs a midroll commercial break, either manually or automatically via ads manager.

use super::*;
/// [`channel.ad_break.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelad_breakbegin): a user runs a midroll commercial break, either manually or automatically via ads manager.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelAdBreakBeginBeta {
    /// The ID of the broadcaster that you want to get Channel Ad Break begin notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelAdBreakBeginBeta {
    /// The ID of the broadcaster that you want to get Channel Ad Break begin notifications for.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelAdBreakBeginBeta {
    type Payload = ChannelAdBreakBeginBetaPayload;

    const EVENT_TYPE: EventType = EventType::ChannelAdBreakBegin;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ChannelReadAds,
        twitch_oauth2::Scope::ChannelManageAds
    )];
    const VERSION: &'static str = "beta";
}

/// [`channel.ad_break.begin`](ChannelAdBreakBeginBeta) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelAdBreakBeginBetaPayload {
    /// Length in seconds of the mid-roll ad break requested
    pub length_seconds: i32,
    #[serde(alias = "timestamp")]
    /// The UTC timestamp of when the ad break began, in RFC3339 format. Note that there is potential delay between this event, when the streamer requested the ad break, and when the viewers will see ads.
    pub started_at: types::Timestamp,
    /// Indicates if the ad was automatically scheduled via Ads Manager
    pub is_automatic: bool,
    /// The broadcaster’s user ID for the channel the ad was run on.
    pub broadcaster_user_id: types::UserId,
    /// The ID of the user that requested the ad. For automatic ads, this will be the ID of the broadcaster.
    pub requester_user_id: types::UserId,
    /// The broadcaster’s user login for the channel the ad was run on.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster’s user display name for the channel the ad was run on.
    pub broadcaster_user_name: types::DisplayName,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    // FIXME: twitch docs has trailing commas
    // FIXME: it uses string for the integer and bool, https://github.com/twitchdev/issues/issues/857#issuecomment-1793796590
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.ad_break.begin",
            "version": "beta",
            "status": "enabled",
            "cost": 0,
            "condition": {
               "broadcaster_user_id": "1337"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.634234626Z"
        },
        "event": {
            "length_seconds": 60,
            "started_at": "2019-11-16T10:11:12.634234626Z",
            "is_automatic": false,
            "broadcaster_user_id": "1337",
            "requester_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User"
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}
