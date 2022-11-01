#![doc(alias = "channel.charity_campaign.donate")]
//! Sends an event notification when a user donates to the broadcaster’s charity campaign.

use super::*;
/// [`channel.charity_campaign.donate`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelcharity_campaigndonate): a user donates to the broadcaster’s charity campaign.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
pub struct ChannelCharityCampaignDonateBeta {
    /// The ID of the broadcaster that you want to receive notifications about when users donate to their campaign.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelCharityCampaignDonateBeta {
    /// The ID of the broadcaster to get notified about.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelCharityCampaignDonateBeta {
    type Payload = ChannelCharityCampaignDonateBetaPayload;

    const EVENT_TYPE: EventType = EventType::ChannelCharityCampaignDonate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadCharity];
    const VERSION: &'static str = "beta";
}

/// [`channel.charity_campaign.donate`](ChannelCharityCampaignDonateBeta) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelCharityCampaignDonateBetaPayload {
    /// An ID that uniquely identifies the charity campaign.
    pub campaign_id: types::CharityCampaignId,
    /// An ID that uniquely identifies the broadcaster that’s running the campaign.
    pub broadcaster_id: types::UserId,
    /// The broadcaster’s login name.
    pub broadcaster_login: types::UserName,
    /// The broadcaster’s display name.
    pub broadcaster_name: types::DisplayName,
    /// An ID that uniquely identifies the user that donated to the campaign.
    pub user_id: types::UserId,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// The user’s display name.
    pub user_name: types::DisplayName,
    /// An object that contains the amount of the user’s donation.
    pub amount: crate::extra::DonationAmount,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
          "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
          "type": "channel.charity_campaign.donate",
          "version": "beta",
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
          "campaign_id": "123-abc-456-def",
          "broadcaster_id": "123456",
          "broadcaster_name": "SunnySideUp",
          "broadcaster_login": "sunnysideup",
          "user_id": "654321",
          "user_name": "GenerousUser1",
          "user_login": "generoususer1",
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
