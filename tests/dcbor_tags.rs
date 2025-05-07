use cbor_book::*;
use anyhow::Result;
use dcbor::prelude::*;

#[test]
#[rustfmt::skip]
fn example_2() -> Result<()> {

// ANCHOR: example_2
let usd = CBOR::to_tagged_value(TAG_CURRENCY_CODE, "USD");
// ANCHOR_END: example_2

// ANCHOR: example_3
let (tag, item) = usd.clone().try_into_tagged_value()?;
assert_eq!(tag.value(), TAG_CURRENCY_CODE);
assert_eq!(item.try_into_text()?, "USD");
// ANCHOR_END: example_3

// ANCHOR: example_4
let diagnostic = usd.diagnostic();
let expected_diagnostic = r#"

33000("USD")

"#.trim();

assert_eq!(diagnostic, expected_diagnostic);
// ANCHOR_END: example_4

// ANCHOR: example_5
let item = usd
    .try_into_expected_tagged_value(TAG_CURRENCY_CODE)?
    .try_into_text()?;
assert_eq!(item, "USD");

// ANCHOR_END: example_5
Ok(())
}
// 33000("USD")

#[test]
#[rustfmt::skip]
fn decimal_fraction_cbor() {
// ANCHOR: example_8
let a = DecimalFraction::new(-1, 11);
let cbor = a.to_cbor();
assert_eq!(cbor.diagnostic(), r#"

4(
    [-1, 11]
)

"#.trim());
// ANCHOR_END: example_8
}

#[test]
#[rustfmt::skip]
fn decimal_fraction_cbor_roundtrip() -> Result<()> {
// ANCHOR: example_10
// Create a DecimalFraction
let a = DecimalFraction::new(-1, 11);
assert_eq!(a.to_string(), "1.1");

// Convert to CBOR
let cbor = a.clone().to_cbor();

// Convert back to DecimalFraction
let b: DecimalFraction = cbor.try_into()?;

// Check that the original and round-tripped values are equal
assert_eq!(a, b);
// ANCHOR_END: example_10
Ok(())
}


#[test]
#[rustfmt::skip]
fn currency_code_cbor() -> Result<()> {
// ANCHOR: example_12
let usd = CurrencyCode::new("USD");
let cbor = usd.to_cbor();
let usd2: CurrencyCode = cbor.try_into()?;
assert_eq!(usd, usd2);
// ANCHOR_END: example_12
Ok(())
}

#[test]
#[rustfmt::skip]
fn currency_amount_cbor_named() -> Result<()> {
// ANCHOR: example_18
// Register our tags first thing
register_tags();

// Create a CurrencyAmount
let currency_amount = CurrencyAmount::new(
    CurrencyCode::new("USD"),
    DecimalFraction::new(-1, 11)
);
assert_eq!(currency_amount.to_string(), "USD 1.1");

// Convert to CBOR
let cbor = currency_amount.to_cbor();

// Check the diagnostic notation, now with named tags
let expected_diagnostic = r#"

33001(   / CurrencyAmount /
    [
        33000("USD"),   / CurrencyCode /
        4(   / DecimalFraction /
            [-1, 11]
        )
    ]
)

"#.trim();
assert_eq!(cbor.diagnostic_annotated(), expected_diagnostic);

// Convert to binary CBOR data
let data = cbor.to_cbor_data();

// Check the hex representation of the binary data, now with named tags
let expected_hex = r#"

d9 80e9                 # tag(33001) CurrencyAmount
    82                  # array(2)
        d9 80e8         # tag(33000) CurrencyCode
            63          # text(3)
                555344  # "USD"
        c4              # tag(4) DecimalFraction
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
// ANCHOR_END: example_18
Ok(())
}

#[test]
#[rustfmt::skip]
fn debug_and_display_formats() -> Result<()> {
// ANCHOR: example_19
let currency_amount = CurrencyAmount::new(
    CurrencyCode::new("USD"),
    DecimalFraction::new(-1, 11)
);

//
// Using the `Debug` implementation on `CurrencyAmount`
//
let expected_debug = r#"

CurrencyAmount(CurrencyCode("USD"), DecimalFraction { exponent: -1, mantissa: 11 })

"#.trim();
assert_eq!(format!("{:?}", currency_amount), expected_debug);

//
// Using the `Display` implementation on `CurrencyAmount`
//
let expected_display = r#"

USD 1.1

"#.trim();
assert_eq!(format!("{}", currency_amount), expected_display);

let cbor = currency_amount.to_cbor();

//
// Using the `Debug` implementation on `CBOR`
//
let expected_debug_cbor = r#"

tagged(33001, array([tagged(33000, text("USD")), tagged(4, array([negative(-1), unsigned(11)]))]))

"#.trim();
assert_eq!(format!("{:?}", cbor), expected_debug_cbor);

//
// Using the `Display` implementation on `CBOR`
//
let expected_display_cbor = r#"

33001([33000("USD"), 4([-1, 11])])

"#.trim();
assert_eq!(format!("{}", cbor), expected_display_cbor);
// ANCHOR_END: example_19

Ok(())
}
