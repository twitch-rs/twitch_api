#![doc(alias = "extension.bits_transaction.create")]
//! a user purchases a product in the extension.

use super::*;
use crate::eventsub::{EventSubscription, EventType};

/// [`extension.bits_transaction.create`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#extensionbits_transactioncreate): a new transaction is created for a Twitch Extension.

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ExtensionBitsTransactionCreateV1 {
    /// The ID of the extension client that you want to get bits transaction notifications for
    pub extension_client_id: types::ExtensionId,
}

impl ExtensionBitsTransactionCreateV1 {
    /// Get notifications when transactions are created for this extension client
    pub fn new(extension_client_id: impl Into<types::ExtensionId>) -> Self {
        Self {
            extension_client_id: extension_client_id.into(),
        }
    }
}

impl EventSubscription for ExtensionBitsTransactionCreateV1 {
    type Payload = ExtensionBitsTransactionCreateV1Payload;

    const EVENT_TYPE: EventType = EventType::ExtensionBitsTransactionCreate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
    const VERSION: &'static str = "1";
}

/// [`extension.bits_transaction.create`](ExtensionBitsTransactionCreateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ExtensionBitsTransactionCreateV1Payload {
    /// The ID of the bits transaction that was created
    pub id: types::BitsTransactionId,
    /// The ID of the extension client that created the bits transaction
    pub extension_client_id: types::ExtensionClientId,

    /// The transaction’s broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster’s login name.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster’s display name.
    pub broadcaster_user_name: types::DisplayName,

    /// The transaction’s user ID.
    pub user_id: types::UserId,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// The user’s display name.
    pub user_name: types::DisplayName,

    /// The product that was purchased in the transaction
    pub product: ExtensionBitsProduct,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::Event;

    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "extension.bits_transaction.create",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "extension_client_id": "deadbeef"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.634234626Z"
        },
        "event": {
            "id": "bits-tx-id",
            "extension_client_id": "deadbeef",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "user_name": "Coolest_User",
            "user_login": "coolest_user",
            "user_id": "1236",
            "product": {
                "name": "great_product",
                "sku": "skuskusku",
                "bits": 1234,
                "in_development": false
            }
        }
    }"##;
    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ExtensionBitsTransactionCreateV1(_val) = val else {
        panic!("invalid event type");
    };
}
