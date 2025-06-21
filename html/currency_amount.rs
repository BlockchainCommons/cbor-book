use dcbor::prelude::*;
use crate::{ CurrencyCode, DecimalFraction, TAG_CURRENCY_AMOUNT };

#[derive(Clone, Debug, PartialEq, Eq)]
// ANCHOR: example_14
pub struct CurrencyAmount(CurrencyCode, DecimalFraction);

impl CurrencyAmount {
    pub fn new(currency: CurrencyCode, amount: DecimalFraction) -> Self {
        Self(currency, amount)
    }

    pub fn currency(&self) -> &CurrencyCode {
        &self.0
    }

    pub fn amount(&self) -> &DecimalFraction {
        &self.1
    }
}

impl From<CurrencyAmount> for CBOR {
    fn from(value: CurrencyAmount) -> Self {
        let v = vec![value.currency().to_cbor(), value.amount().to_cbor()].to_cbor();
        CBOR::to_tagged_value(TAG_CURRENCY_AMOUNT, v)
    }
}

impl TryFrom<CBOR> for CurrencyAmount {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        let item = cbor.try_into_expected_tagged_value(TAG_CURRENCY_AMOUNT)?;
        let arr = item.try_into_array()?;

        if arr.len() != 2 {
            return Err("Expected a two-element array".into());
        }

        let currency: CurrencyCode = arr[0].clone().try_into()?;
        let amount: DecimalFraction = arr[1].clone().try_into()?;

        Ok(CurrencyAmount(currency, amount))
    }
}

impl std::fmt::Display for CurrencyAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.currency(), self.amount())
    }
}
// ANCHOR_END: example_14
