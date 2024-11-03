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
    pub fn formatted_value(&self) -> Option<String> {
        let currencies = currency_map();
        let currency = currencies.get(&self.currency_code)?;
        Money::from_minor_units(self.base_value, currency)
            .format(&Formatter::default())
            .ok()
    }
}
