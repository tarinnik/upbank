use crate::currencies::currency_map;
use doubloon::{formatter::Formatter, Money};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Links {
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SelfLink {
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Deserialize, Debug)]
pub struct Amount {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    pub value: String,
    #[serde(rename = "valueInBaseUnits")]
    pub base_value: i64,
}

impl Amount {
    /// Formats the amount with the correct currency symbol
    pub fn formatted_value(&self) -> Option<String> {
        format_amount(self.base_value, &self.currency_code)
    }

    /// Formats the absoluate value of the amount with correct
    /// currency symbol
    pub fn formatted_abs_value(&self) -> Option<String> {
        let amount = self.base_value.abs();
        format_amount(amount, &self.currency_code)
    }
}

fn format_amount(amount: i64, currency_code: &String) -> Option<String> {
    let currencies = currency_map();
    let currency = currencies.get(currency_code)?;
    Money::from_minor_units(amount, currency)
        .format(&Formatter::default())
        .ok()
}
