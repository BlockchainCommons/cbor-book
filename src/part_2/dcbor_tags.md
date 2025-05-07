# dCBOR Tags

As discussed in [Part I: CBOR Tags](../part_1/cbor_tags.md), CBOR tags are an integer that "tags" the CBOR data item that follows, specifying its type or meaning. This is a powerful feature of CBOR.

Let's say we wanted to define a tag that identifies a string as holding an ISO 4217 currency code like `USD` or `EUR`. We could just use a bare string, but if we want our type to be completely self-describing, we can define a tag for it.

As long as you are the only one using that tag, you can choose any integer you want. But if you want your structure to interoperate with other systems, you should use a tag that is registered with IANA, discussed previously [here](../part_1/cbor_tags.md#how-to-register-your-own-fcfs-tags).

For our demonstration we'll use the tag `33000`, which as of this writing is unassigned by IANA.

How would we tag a string as a currency type? Let's start by defining a constant for our tag:

```rust
const TAG_CURRENCY_CODE: u64 = 33000;
```

We now associate our string with the tag by using the `to_tagged_value()` method:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_2}}
```

We can extract the tag and the tagged value using `try_into_tagged_value()`. The return type is a tuple of a `Tag` and the tagged item:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_3}}
```

The reason you have to call `value()` on the returned `Tag` to get back the numeric value is that a `Tag` may also include a human-readable name you can define for your tag. We'll discuss naming tags later in this chapter.

> ✅ **NOTE:** A _tagged value_ is the combination of a tag and the value (data item) it tags. But the _value of the tag_ is the integer that identifies the tag.

If we print the diagnostic notation of our tagged value, we can see the tag in the output:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_4}}
```

As shown above, we can always extract the `(Tag, CBOR)` tuple from a tagged value, and then compare the tag value to our constant to see whether we want to process it further. But it's a common pattern to expect to find a specific tag in a particular place in a CBOR structure. `dcbor` provides a convenience method `try_into_expected_tagged_value()` to test the tag value and return an error if it doesn't match. If it succeeds, it returns the tagged value for further processing.

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_5}}
```

## Tagging a Complex Structure

Let's say we want to combine our tagged currency code with an amount. Currency amounts can be tricky, because they are expressed as having decimal fractions, but many common floating point values, like `1.1` cannot be represented exactly in binary floating point, meaning that even highly-precise types like `f64` can't represent common currency values accurately.

Let's define a new type called `DecimalFraction` that holds an integer mantissa and a signed integer exponent representing powers of 10. When negative, the exponent indicates the number of places to the right of the decimal point, so `1.1` would be represented as a mantissa of `11` with an exponent of `-1`, and `1.01` would be represented as a mantissa of `101` with an exponent of `-2`.

```rust
{{#rustdoc_include ../decimal_fraction.rs:example_6}}
```

> ✅ **NOTE:** We're not showing a lot of the typical boilerplate code here, like the `impl`s for `Debug`, `Clone`, `Display`, and things like `new()` methods. You can find the complete code in the repo for this book.

It turns out that [RFC8949 §3.4.4](https://www.rfc-editor.org/rfc/rfc8949.html#name-decimal-fractions-and-bigfl) already defines a CBOR schema for decimal fractions, so we can use that: it's just a two-element array with the exponent first and the mantissa second. It also reserves the tag `4` for decimal fractions, so we can use that as our tag.

```rust
const TAG_DECIMAL_FRACTION: u64 = 4;
```

```rust
{{#rustdoc_include ../decimal_fraction.rs:example_7}}
```

Now we can create a `DecimalFraction` and convert it to CBOR, showing the diagnostic notation:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_8}}
```

Because conversion from CBOR to a given type can fail, we implement the `TryFrom<CBOR>` trait for our `DecimalFraction` type:

```rust
{{#rustdoc_include ../decimal_fraction.rs:example_9}}
```

Now we can round-trip our tagged value, converting it to CBOR and back to a `DecimalFraction`:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_10}}
```

## Implementing a Tagged String

We used a tagged string for our currency code, but we can also define a `CurrencyCode` type using the _newtype_ pattern. This is a common Rust idiom for creating a new type that wraps an existing type, like `String`, and provides additional functionality. In this case, the additional functionality is to implement `From<CurrencyCode> for CBOR` and `TryFrom<CBOR> for CurrencyCode`.

```rust
{{#rustdoc_include ../currency_code.rs:example_11}}
```

Now we can round-trip our `CurrencyCode` the same way we did with `DecimalFraction`:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_12}}
```

## Combining the Two Types

Originally we set out to create a structure that combined a currency code with a decimal fraction: `CurrencyAmount`. We'd also like this structure to have its own tag, so we'll use `33001`, which is also unassigned by IANA as of this writing.

```rust
const TAG_CURRENCY_AMOUNT: u64 = 33001;
```

Now that we have completely reusable constituents, we can define `CurrencyAmount` as a type that consists of a `CurrencyCode` and a `DecimalFraction`.

```rust
{{#rustdoc_include ../currency_amount.rs:example_14}}
```

Notice that in the above example, we're able to call the `to_cbor()` method on the `CurrencyCode` and `DecimalFraction` types, because the `dcbor` library includes a _blanket implementation_ for another trait called `CBOREncodable`, which automatically applies to any type that implements `Into<CBOR>` and `Clone`. (We implemented `From<CurrencyCode> for CBOR` and `From<DecimalFraction> for CBOR` which also implicitly implement the `Into<CBOR>` trait, so we get the `CBOREncodable` trait for free.)

The `CBOREncodable` trait gives us the `to_cbor()` method, which can be called on a `&self` (reference to self) unlike the `into()` method, which consumes the value. It also gives us the `to_cbor_data()` method, which returns the final, serialized CBOR data as a `Vec<u8>`.

This use of blanket implementations is a common Rust idiom, similar to how types that implement the `Display` trait automatically implement the `ToString` trait and hence gain the `to_string()` method.

Now with all the pieces in place, we can do a full round-trip of our `CurrencyAmount` type:

```rust
{{#rustdoc_include ../../tests/currency_amount_no_names.rs:example_15}}
```

## Named Tags

As mentioned, a CBOR tag is just an integer, and that integer is all that is ever serialized to the binary stream. But the `dcbor` library allows you to associate a human-readable name with a tag, which can be useful for debugging and documentation. The `dcbor` library provides a macro for defining compile-time constants for tags and their names:

```rust
{{#rustdoc_include ../tags.rs:example_16}}
```

These macro invocations are a concise equivalent to the following code:

```rust
const TAG_DECIMAL_FRACTION: u64 = 4;
const TAG_NAME_DECIMAL_FRACTION: &str = "DecimalFraction";

const TAG_CURRENCY_CODE: u64 = 33000;
const TAG_NAME_CURRENCY_CODE: &str = "CurrencyCode";

const TAG_CURRENCY_AMOUNT: u64 = 33001;
const TAG_NAME_CURRENCY_AMOUNT: &str = "CurrencyAmount";
```

To make these names available to runtime calls like `CBOR::diagnostic_annotated` and `CBOR::hex_annotated`, we need to _register_ them once at the start of our program:

```rust
{{#rustdoc_include ../tags.rs:example_17}}
```

The `cbor_tag!` macro is actually doing the work of creating the `Tag` instances for us, using the same naming convention as the constants defined using the `const_cbor_tag!` macro. The `with_tags_mut!` macro provides writable, thread-safe access to the global tag registry.

Here's the same example as before, but calling `register_tags()` at the start of the program. Now both output formats include the human-readable names for the tags:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_18}}
```

## A Note About the `Debug` and `Display` implementations on `CBOR`

You've been learning about calls like `CBOR::diagnostic_annotated()` and `CBOR::hex_annotated()`, which are used to print the CBOR data in a human-readable format, and `CBOR::to_cbor_data()`, which returns the raw CBOR data as a `Vec<u8>`.

These methods are useful for debugging (and of course serializing your CBOR), but they are not the same as the `Debug` and `Display` traits, and it's also important to understand the difference between how these trait outputs are formatted on your _original_ structures, versus how they are formatted on the `CBOR` type:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_19}}
```

- The `Debug` trait on `CurrencyAmount` is just the default `Debug` implementation for a struct, which prints the field names and values in a human-readable format.
- The `Display` trait on `CurrencyAmount` is a custom implementation that formats the value as a string with the currency code and amount.
- The `Debug` trait on `CBOR` is a nested symbolic representation of the CBOR major types and values.
- The `Display` trait on `CBOR` is the same as would be returned by `CBOR::diagnostic_flat()`, which is valid diagnostic notation all on one line.

Each of these formats is useful in its own way, so knowing when to use each one will help you get the most out of the `dcbor` library.
