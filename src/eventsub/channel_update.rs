#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelUpdate {
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelUpdate {
    const TYPE: EventType = EventType::ChannelUpdate;
    const VERSION: &'static str = "1";

    fn condition(&self) -> Result<serde_json::Value, ()> { todo!() }
}

pub struct ChannelUpdatePayload {
    /// The broadcaster’s user ID.
    broadcaster_user_id: types::UserId,
    /// The broadcaster’s user name.
    broadcaster_user_name: types::UserName,
    /// The channel’s stream title.
    title: String,
    /// The channel’s broadcast language.
    language: String,
    /// The channel’s category ID.
    category_id: types::CategoryId,
    /// The category name.
    category_name: String,
    /// A boolean identifying whether the channel is flagged as mature. Valid values are true and false.
    is_mature: bool,
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
