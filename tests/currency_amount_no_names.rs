use cbor_book::*;
use anyhow::Result;
use dcbor::prelude::*;

#[test]
#[rustfmt::skip]
fn currency_amount_no_names() -> Result<()> {
// ANCHOR: example_15
// Create a CurrencyAmount
let currency_amount = CurrencyAmount::new(
    CurrencyCode::new("USD"),
    DecimalFraction::new(-1, 11)
);
assert_eq!(currency_amount.to_string(), "USD 1.1");

// Convert to CBOR
let cbor = currency_amount.to_cbor();

// Check the diagnostic notation
let expected_diagnostic = r#"
33001(
    [
        33000("USD"),
        4(
            [-1, 11]
        )
    ]
)
"#.trim();
assert_eq!(cbor.diagnostic_annotated(), expected_diagnostic);

// Convert to binary CBOR data
let data = cbor.to_cbor_data();

// Check the hex representation of the binary data
let expected_hex = r#"

d9 80e9                 # tag(33001)
    82                  # array(2)
        d9 80e8         # tag(33000)
            63          # text(3)
                555344  # "USD"
        c4              # tag(4)
            82          # array(2)
                20      # negative(-1)
                0b      # unsigned(11)

"#.trim();
assert_eq!(cbor.hex_annotated(), expected_hex);

// Convert back to CBOR
let cbor2 = CBOR::try_from_data(data)?;

// Convert back to CurrencyAmount
let currency_amount2: CurrencyAmount = cbor2.try_into()?;

// Check that the original and round-tripped values are equal
assert_eq!(currency_amount, currency_amount2);
// ANCHOR_END: example_15
Ok(())
}
