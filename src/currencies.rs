use doubloon::{currency_map::CurrencyMap, iso_currencies::*, Currency};

pub fn currency_map() -> CurrencyMap<'static> {
    CurrencyMap::from_collection(vec![&AUD as &dyn Currency])
}
