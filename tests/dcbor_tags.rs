use anyhow::Result;
use std::fmt;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// ANCHOR: example_6
pub struct DecimalFraction {
    pub exponent: i8,
    pub mantissa: i64,
}
// ANCHOR_END: example_6

impl DecimalFraction {
    /// Create a new `DecimalFraction` from raw parts.
    pub fn new(exponent: i8, mantissa: i64) -> Self {
        Self { exponent, mantissa }
    }

    /// Convert back to `f64`. May lose precision on large exponents.
    pub fn to_f64(self) -> f64 {
        (self.mantissa as f64) * (10f64).powi(self.exponent as i32)
    }
}

impl fmt::Display for DecimalFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mantissa == 0 {
            return write!(f, "0");
        }

        let abs_value = self.mantissa.abs();
        let is_negative = self.mantissa < 0;
        let prefix = if is_negative { "-" } else { "" };

        if self.exponent >= 0 {
            // For positive exponent, add zeros after the number
            write!(f, "{}{}{}", prefix, abs_value, "0".repeat(self.exponent as usize))
        } else {
            // For negative exponent, insert decimal point
            let abs_exp = -self.exponent as usize;
            let value_str = abs_value.to_string();

            if value_str.len() <= abs_exp {
                // Decimal point at the beginning with possible leading zeros
                let padding = abs_exp - value_str.len();
                write!(f, "{}0.{}{}", prefix, "0".repeat(padding), value_str)
            } else {
                // Insert decimal point within the number
                let decimal_pos = value_str.len() - abs_exp;
                let (integer_part, fractional_part) = value_str.split_at(decimal_pos);
                write!(f, "{}{}.{}", prefix, integer_part, fractional_part)
            }
        }
    }
}

// ANCHOR: example_7

impl From<DecimalFraction> for CBOR {
    fn from(value: DecimalFraction) -> Self {
        // Compose the two-element array
        let v: CBOR = vec![value.exponent as i64, value.mantissa].into();

        // Return the tagged array
        CBOR::to_tagged_value(TAG_DECIMAL_FRACTION, v)
    }
}

// ANCHOR_END: example_7

#[test]
#[rustfmt::skip]
fn decimal_fraction_cbor() {
// ANCHOR: example_8

let a = DecimalFraction::new(-1, 11);
let cbor: CBOR = a.into();
assert_eq!(cbor.diagnostic(), r#"

4(
    [-1, 11]
)

"#.trim());

// ANCHOR_END: example_8
}

// ANCHOR: example_9

impl TryFrom<CBOR> for DecimalFraction {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        // Decode the tagged array
        let item = cbor.try_into_expected_tagged_value(TAG_DECIMAL_FRACTION)?;

        // Convert the item to an array
        let arr = item.try_into_array()?;

        // Validate the length of the array
        if arr.len() != 2 {
            return Err("Expected a two-element array".into());
        }

        // Extract the exponent and mantissa
        let exponent: i8 = arr[0].clone().try_into()?;
        let mantissa: i64 = arr[1].clone().try_into()?;

        // Return the DecimalFraction
        Ok(DecimalFraction::new(exponent, mantissa))
    }
}

// ANCHOR_END: example_9

#[test]
#[rustfmt::skip]
fn decimal_fraction_cbor_roundtrip() -> Result<()> {
// ANCHOR: example_10

// Create a DecimalFraction
let a = DecimalFraction::new(-1, 11);
assert_eq!(a.to_string(), "1.1");

// Convert to CBOR
let cbor: CBOR = a.clone().into();

// Convert back to DecimalFraction
let b: DecimalFraction = cbor.try_into()?;

// Check that the original and round-tripped values are equal
assert_eq!(a, b);

// ANCHOR_END: example_10
Ok(())
}

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


#[test]
#[rustfmt::skip]
fn currency_code_cbor() -> Result<()> {
// ANCHOR: example_12

let usd = CurrencyCode::new("USD");
let cbor: CBOR = usd.clone().into();
let usd2: CurrencyCode = cbor.try_into()?;
assert_eq!(usd, usd2);

// ANCHOR_END: example_12
Ok(())
}

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
        let v: CBOR = vec![
            value.currency().to_cbor(),
            value.amount().to_cbor()
        ].into();
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


#[test]
#[rustfmt::skip]
fn currency_amount_cbor() -> Result<()> {
// ANCHOR: example_15

// Create a CurrencyAmount
let currency_amount = CurrencyAmount::new(
    CurrencyCode::new("USD"),
    DecimalFraction::new(-1, 11)
);
assert_eq!(currency_amount.to_string(), "USD 1.1");

// Convert to CBOR
let cbor: CBOR = currency_amount.clone().into();

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
let cbor: CBOR = currency_amount.clone().into();

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


#[test]
fn decimal_fraction_1() {
    let a = DecimalFraction::new(-1, 11);
    assert_eq!(a.mantissa, 11);
    assert_eq!(a.exponent, -1);
    assert!((a.to_f64() - 1.1).abs() < f64::EPSILON);

    let b = DecimalFraction::new(-2, 101);
    assert_eq!(b.mantissa, 101);
    assert_eq!(b.exponent, -2);
    assert!((b.to_f64() - 1.01).abs() < f64::EPSILON);
}

#[test]
fn decimal_fraction_display() {
    // Test zero
    let zero = DecimalFraction::new(0, 0);
    assert_eq!(zero.to_string(), "0");

    // Test positive value with zero exponent
    let simple = DecimalFraction::new(0, 42);
    assert_eq!(simple.to_string(), "42");

    // Test positive values with positive exponent
    let pos_exp1 = DecimalFraction::new(2, 5);
    assert_eq!(pos_exp1.to_string(), "500");

    let pos_exp2 = DecimalFraction::new(3, 123);
    assert_eq!(pos_exp2.to_string(), "123000");

    // Test negative values with positive exponent
    let neg_pos_exp = DecimalFraction::new(1, -42);
    assert_eq!(neg_pos_exp.to_string(), "-420");

    // Test positive values with negative exponent
    let pos_neg_exp1 = DecimalFraction::new(-2, 123);
    assert_eq!(pos_neg_exp1.to_string(), "1.23");

    let pos_neg_exp2 = DecimalFraction::new(-1, 5);
    assert_eq!(pos_neg_exp2.to_string(), "0.5");

    let pos_neg_exp3 = DecimalFraction::new(-3, 5);
    assert_eq!(pos_neg_exp3.to_string(), "0.005");

    // Test negative values with negative exponent
    let neg_neg_exp1 = DecimalFraction::new(-2, -123);
    assert_eq!(neg_neg_exp1.to_string(), "-1.23");

    let neg_neg_exp2 = DecimalFraction::new(-3, -5);
    assert_eq!(neg_neg_exp2.to_string(), "-0.005");

    // Test boundary cases
    let boundary1 = DecimalFraction::new(-9, 123456789);
    assert_eq!(boundary1.to_string(), "0.123456789");

    let boundary2 = DecimalFraction::new(-1, 1);
    assert_eq!(boundary2.to_string(), "0.1");
}
