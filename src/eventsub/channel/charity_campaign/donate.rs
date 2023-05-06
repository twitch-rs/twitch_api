#![doc(alias = "channel.charity_campaign.donate")]
//! Sends an event notification when a user donates to the broadcaster’s charity campaign.

use super::*;
/// [`channel.charity_campaign.donate`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelcharity_campaigndonate): a user donates to the broadcaster’s charity campaign.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
pub struct ChannelCharityCampaignDonateV1 {
    /// The ID of the broadcaster that you want to receive notifications about when users donate to their campaign.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelCharityCampaignDonateV1 {
    /// The ID of the broadcaster to get notified about.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelCharityCampaignDonateV1 {
    type Payload = ChannelCharityCampaignDonateV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelCharityCampaignDonate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelReadCharity];
    const VERSION: &'static str = "1";
}

/// [`channel.charity_campaign.donate`](ChannelCharityCampaignDonateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelCharityCampaignDonateV1Payload {
    /// An ID that identifies the donation. The ID is unique across campaigns.
    pub id: types::CharityDonationId,
    /// An ID that identifies the charity campaign.
    pub campaign_id: types::CharityCampaignId,
    /// An ID that identifies the broadcaster that’s running the campaign.
    #[serde(alias = "broadcaster_user_id")]
    pub broadcaster_id: types::UserId,
    /// An ID that identifies the charity campaign.
    #[serde(alias = "broadcaster_user_login")]
    pub broadcaster_login: types::UserName,
    /// An ID that identifies the broadcaster that’s running the campaign.
    #[serde(alias = "broadcaster_user_name")]
    pub broadcaster_name: types::DisplayName,
    /// An ID that identifies the user that donated to the campaign.
    pub user_id: types::UserId,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// The user’s display name.
    pub user_name: types::DisplayName,
    /// The charity’s name.
    pub charity_name: String,
    /// A description of the charity.
    pub charity_description: String,
    /// A URL to an image of the charity’s logo. The image’s type is PNG and its size is 100px X 100px.
    pub charity_logo: String,
    /// A URL to the charity’s website.
    pub charity_website: String,
    /// An object that contains the amount of money that the user donated.
    pub amount: crate::extra::DonationAmount,
}

#[cfg(test)]
#[test]
fn parse_payload_correct_maybe() {
    let payload = r##"
    {
      "subscription": {
        "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        "type": "channel.charity_campaign.donate",
        "version": "1",
        "status": "enabled",
        "cost": 0,
        "condition": {
          "broadcaster_user_id": "123456"
        },
        "transport": {
          "method": "webhook",
          "callback": "https://example.com/webhooks/callback"
        },
        "created_at": "2022-07-25T10:11:12.123Z"
      },
      "event": {
        "id": "a1b2c3-aabb-4455-d1e2f3",
        "campaign_id": "123-abc-456-def",
        "broadcaster_user_id": "123456",
        "broadcaster_user_name": "SunnySideUp",
        "broadcaster_user_login": "sunnysideup",
        "user_id": "654321",
        "user_login": "generoususer1",
        "user_name": "GenerousUser1",
        "charity_name": "Example name",
        "charity_description": "Example description",
        "charity_logo": "https://abc.cloudfront.net/ppgf/1000/100.png",
        "charity_website": "https://www.example.com",
        "amount": {
          "value": 10000,
          "decimal_places": 2,
          "currency": "USD"
        }
      }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
      {
        "subscription": {
          "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
          "type": "channel.charity_campaign.donate",
          "version": "1",
          "status": "enabled",
          "cost": 0,
          "condition": {
            "broadcaster_user_id": "123456"
          },
          "transport": {
            "method": "webhook",
            "callback": "https://example.com/webhooks/callback"
          },
          "created_at": "2022-07-25T10:11:12.123Z"
        },
        "event": {
          "id": "a1b2c3-aabb-4455-d1e2f3",
          "campaign_id": "123-abc-456-def",
          "broadcaster_id": "123456",
          "broadcaster_name": "SunnySideUp",
          "broadcaster_login": "sunnysideup",
          "user_id": "654321",
          "user_login": "generoususer1",
          "user_name": "GenerousUser1",
          "charity_name": "Example name",
          "charity_description": "Example description",
          "charity_logo": "https://abc.cloudfront.net/ppgf/1000/100.png",
          "charity_website": "https://www.example.com",
          "amount": {
            "value": 10000,
            "decimal_places": 2,
            "currency": "USD"
          }
        }
      }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}
