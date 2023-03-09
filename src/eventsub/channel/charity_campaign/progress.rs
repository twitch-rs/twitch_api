#![doc(alias = "channel.charity_campaign.progress")]
//! Sends an event notification when progress is made towards the campaign’s goal or when the broadcaster changes the fundraising goal.

use super::*;
/// [`channel.charity_campaign.progress`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelcharity_campaignprogress): progress is made towards the campaign’s goal or when the broadcaster changes the fundraising goal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
pub struct ChannelCharityCampaignProgressV1 {
    /// The ID of the broadcaster that you want to receive notifications about when their campaign makes progress or is updated.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelCharityCampaignProgressV1 {
    /// The ID of the broadcaster to get notified about.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelCharityCampaignProgressV1 {
    type Payload = ChannelCharityCampaignProgressV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelCharityCampaignProgress;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadCharity];
    const VERSION: &'static str = "1";
}

/// [`channel.charity_campaign.progress`](ChannelCharityCampaignProgressV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelCharityCampaignProgressV1Payload {
    /// An ID that identifies the charity campaign.
    pub id: types::CharityCampaignId,
    /// An ID that identifies the broadcaster that’s running the campaign.
    pub broadcaster_id: types::UserId,
    /// An ID that identifies the charity campaign.
    pub broadcaster_login: types::UserName,
    /// An ID that identifies the broadcaster that’s running the campaign.
    pub broadcaster_name: types::DisplayName,
    /// The charity’s name.
    pub charity_name: String,
    /// A description of the charity.
    pub charity_description: String,
    /// A URL to an image of the charity’s logo. The image’s type is PNG and its size is 100px X 100px.
    pub charity_logo: String,
    /// A URL to the charity’s website.
    pub charity_website: String,
    /// An object that contains the current amount of donations that the campaign has received.
    pub current_amount: crate::extra::DonationAmount,
    /// An object that contains the campaign’s target fundraising goal.
    pub target_amount: crate::extra::DonationAmount,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
      "subscription": {
        "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        "type": "channel.charity_campaign.progress",
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
        "created_at": "2022-07-25T10:11:12.12339824Z"
      },
      "event": {
        "id": "123-abc-456-def",
        "broadcaster_id": "123456",
        "broadcaster_name": "SunnySideUp",
        "broadcaster_login": "sunnysideup",
        "charity_name": "Example name",
        "charity_description": "Example description",
        "charity_logo": "https://abc.cloudfront.net/ppgf/1000/100.png",
        "charity_website": "https://www.example.com",
        "current_amount": {
          "value": 260000,
          "decimal_places": 2,
          "currency": "USD"
        },
        "target_amount": {
          "value": 1500000,
          "decimal_places": 2,
          "currency": "USD"
        }
      }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}
