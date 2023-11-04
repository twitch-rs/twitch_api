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
