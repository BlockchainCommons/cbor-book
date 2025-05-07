use dcbor::prelude::*;

use crate::TAG_CURRENCY_CODE;

#[derive(Clone, Debug, PartialEq, Eq)]
// ANCHOR: example_11
pub struct CurrencyCode(String);

impl CurrencyCode {
    pub fn new(code: &str) -> Self {
        Self(code.into())
    }

    pub fn code(&self) -> &str {
        &self.0
    }
}

impl From<CurrencyCode> for CBOR {
    fn from(value: CurrencyCode) -> Self {
        CBOR::to_tagged_value(TAG_CURRENCY_CODE, value.0)
    }
}

impl TryFrom<CBOR> for CurrencyCode {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        let value = cbor.try_into_expected_tagged_value(TAG_CURRENCY_CODE)?;
        let currency_code: String = value.try_into()?;
        Ok(CurrencyCode(currency_code))
    }
}

impl std::fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
// ANCHOR_END: example_11
