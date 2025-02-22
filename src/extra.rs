#[cfg(any(feature = "eventsub", feature = "helix"))]
#[derive(Clone, Debug, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
/// Represents a donation "amount"
pub struct DonationAmount {
    /// The monetary amount.
    ///
    /// The amount is specified in the currency’s minor unit. For example, the minor units for USD is cents, so if the amount is $5.50 USD, value is set to 550.
    pub value: i32,
    /// The number of decimal places used by the currency. For example, USD uses two decimal places. Use this number to translate value from minor units to major units by using the formula:
    /// value / 10^decimal_places
    pub decimal_places: i32,
    /// The ISO-4217 three-letter currency code that identifies the type of currency in value.
    pub currency: String,
}

#[cfg(any(feature = "eventsub", feature = "helix"))]
/// Valid colors for announcements
#[derive(PartialEq, Eq, serde_derive::Deserialize, serde_derive::Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AnnouncementColor {
    /// The color blue
    #[serde(alias = "BLUE")]
    Blue,
    /// The color green
    #[serde(alias = "GREEN")]
    Green,
    /// The color orange
    #[serde(alias = "ORANGE")]
    Orange,
    /// The color purple
    #[serde(alias = "PURPLE")]
    Purple,
    /// The primary color for the broadcaster
    #[serde(alias = "PRIMARY")]
    Primary,
}

#[cfg(any(feature = "eventsub", feature = "helix"))]
impl Default for AnnouncementColor {
    fn default() -> Self { Self::Primary }
}

/// An error for an invalid [AnnouncementColor]
#[cfg(any(feature = "eventsub", feature = "helix"))]
#[derive(Debug, Clone, thiserror::Error)]
#[error("invalid color")]
pub struct AnnouncementColorError;

#[cfg(any(feature = "eventsub", feature = "helix"))]
impl std::convert::TryFrom<&str> for AnnouncementColor {
    type Error = AnnouncementColorError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match &*value.to_lowercase() {
            "blue" => Self::Blue,
            "green" => Self::Green,
            "orange" => Self::Orange,
            "purple" => Self::Purple,
            "primary" => Self::Primary,
            _ => return Err(AnnouncementColorError),
        })
    }
}
