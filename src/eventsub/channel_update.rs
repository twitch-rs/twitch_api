use super::*;
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelUpdate {
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelUpdate {
    type Payload = ChannelUpdatePayload;

    const EVENT_TYPE: EventType = EventType::ChannelUpdate;
    const VERSION: &'static str = "1";
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelUpdatePayload {
    /// The broadcaster’s user ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster’s user name.
    pub broadcaster_user_name: types::UserName,
    /// The channel’s stream title.
    pub title: String,
    /// The channel’s broadcast language.
    pub language: String,
    /// The channel’s category ID.
    pub category_id: types::CategoryId,
    /// The category name.
    pub category_name: String,
    /// A boolean identifying whether the channel is flagged as mature. Valid values are true and false.
    pub is_mature: bool,
}

impl NotificationPayload for ChannelUpdatePayload {}

#[test]
fn channel_update() {
    let sub = ChannelUpdate {
        broadcaster_user_id: "27620241".to_string(),
    };
    let t = Transport {
        method: TransportMethod,
        callback: "hello",
        secret: "hello",
    };
    sub.create_subscription_request("1234", "1234")
}
