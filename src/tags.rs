use dcbor::prelude::*;

// ANCHOR: example_16
const_cbor_tag!(4, DECIMAL_FRACTION, "DecimalFraction");
const_cbor_tag!(33000, CURRENCY_CODE, "CurrencyCode");
const_cbor_tag!(33001, CURRENCY_AMOUNT, "CurrencyAmount");
// ANCHOR_END: example_16

// ANCHOR: example_17
pub fn register_tags() {
    with_tags_mut!(|tags_store: &mut TagsStore| {
        tags_store.insert_all(vec![
            cbor_tag!(DECIMAL_FRACTION),
            cbor_tag!(CURRENCY_CODE),
            cbor_tag!(CURRENCY_AMOUNT),
        ]);
    });
}
// ANCHOR_END: example_17
