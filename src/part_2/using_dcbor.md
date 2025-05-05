# Using dCBOR

So after all that discussion of the motivation for dCBOR, let's just recap its rules all in one place, and specifically how they differ from basic CBOR:

- **Map Keys**: No duplicates. Must be serialized sorted lexicographically by the serialized key.
- **Numeric Values**: "Preferred Serialization" isn't just preferred, it's required.
- **Numeric Reduction**: Floating point values that can accurately be represented as integers must be serialized as integers.
- **Indefinite Length**: Indefinite length values are not allowed.
- **Simple Values**: Only `false`, `true`, and `null` are allowed.
- **Strings**: Must be encoded in Unicode Normalization Form C (NFC).
- **Decoders**: Must check all the rules above and reject any serialization that doesn't conform to them.

Pretty simple, right?

It gets even simpler when you use a CBOR library that supports dCBOR directly, as the implementation should take care of all the details for you. In fact, a good API will even make it _impossible_ to create invalid dCBOR serializations.

The [`dcbor`](https://crates.io/crates/dcbor) crate is the Rust reference implementation of dCBOR from Blockchain Commons, and in this chapter we'll show you how easy it is to use.

## Installation

This will add the latest version of the `dcbor` crate to your `Cargo.toml` file:

```bash
cargo add dcbor
```

## Basic Usage

`dcbor` includes a `prelude` module that re-exports all the types and traits you need to use dCBOR:

```rust
{{#rustdoc_include ../../tests/test_1.rs:first}}
```

Many common types are directly convertible into dCBOR. Thanks to dCBOR's numeric reduction, you don't even need to specify whether common numeric types should be serialized as integers or floating point: the `dcbor` library will automatically choose the best representation for you.

```rust
{{#rustdoc_include ../../tests/test_1.rs:second}}
```

When you use `value.to_cbor()` or `CBOR::from(value)`, you're not _actually_ encoding the CBOR serialization in that moment. You're actually creating an intermediate representation of the data (an instance of `CBOR`) that can be serialized later when you call a method like `to_cbor_data`.

Converting back from CBOR is also easy: you specify the type you want to convert to, and the `dcbor` library will do the rest. You use the `try_from` method to convert from CBOR to a Rust type, which will succeed if the CBOR can be accurately converted to that type. If the conversion fails, it will return an error:

```rust
{{#rustdoc_include ../../tests/test_1.rs:test_2}}
```

In the following example we use `try_from` to convert from CBOR to both a `u8` type and an `f64` type. Both succeed, because the value `42` can be represented as both an 8-bit unsigned integer and a 64-bit floating point number:

```rust
{{#rustdoc_include ../../tests/test_1.rs:test_3}}
```

> **✅ NOTE:** Observe the call to `clone()` above, which we need because the `try_from` method consumes the `CBOR` value, and we still need an instance for the second `try_from` call. Instances of `CBOR` are immutable, and the `dcbor` library implements structure sharing, so cloning is always cheap.

Below we encode a floating point value with a non-zero fractional part, which succeeds in being decoded back to floating point, but fails to decode back to an integer, because precision would be lost:

```rust
{{#rustdoc_include ../../tests/test_1.rs:test_4}}
```

The nice thing about this idiom is it's not just for numeric types. You can use it for any type that implements the `TryFrom<CBOR>`, like `String`:

```rust
{{#rustdoc_include ../../tests/test_1.rs:test_5}}
```

It even works for vectors:

```rust
{{#rustdoc_include ../../tests/test_1.rs:test_6}}
```

## Byte Strings

But the last example raises an interesting question: is our `Vec<u8>` being serialized as a CBOR _array_ or a CBOR _byte string_? Let's check:

```rust
{{#rustdoc_include ../../tests/test_1.rs:test_7}}
```

As you can see, the header byte specifies an array of five elements, followed by five CBOR data items for the integers `1`, `2`, `3`, `4`, and `5`. So the `Vec<u8>` is being serialized as a CBOR array, not a byte string.

In Rust, `Vec<u8>` is often used to represent a string or buffer of bytes, but in CBOR, a byte string is a different type distinct from a vector or an array. The `CBOR` type provides a static method `CBOR::to_byte_string` that will convert a `Vec<u8>` into a CBOR byte string:

```rust
{{#rustdoc_include ../../tests/test_1.rs:test_8}}
```

Everything in the serialization in this example is the same as the last, _except_ the header byte, which was `0x85` for a 5-element array, and `0x45` for a byte string of length 5.

Notice that recovering the byte string is also different. Since a byte string is not an array, we can't extract it as a `Vec<u8>`. Instead, we extract it as the type `ByteString`, and then convert _that_ to a `Vec<u8>` using `.into()`.

`ByteString` is just a wrapper around `Vec<u8>`, and it has most of the same capabilities, but the `dcbor` library treats it as a CBOR byte string, not a CBOR array.

## Simple Values: `false`, `true`, and `null`

dCBOR only allows three simple values: `false`, `true`, and `null`, and the `dcbor` library provides a set of conveniences for working with them. In the example below we create a CBOR array containing `[true, false, null]`, and then test its CBOR diagnostic notation and annotated hex serialization:

```rust
{{#rustdoc_include ../../tests/test_1.rs:test_9}}
```

Something interesting is going on here: our array has three values, two of which are booleans and the third is its own type: `null`. CBOR is designed to handle such _heterogenous arrays_ with no problem. But Rust (unlike some languages like JavaScript) doesn't have a `null` value (preferring `Option<T>` for values which may not be present). Rust also doesn't natively support `Vec`s containing mixed types. So how does the `dcbor` library handle this?

First, note that our array is not declared as a `Vec<bool>` but as a `Vec<CBOR>`. The CBOR type can hold *any* cbor value, including complex values like nested arrays and maps. In the context of the `vec!` macro composing a `Vec<CBOR>`, the Rust boolean values `true` and `false` can just be converted directly using `.into()`, and that's what we're doing here.

Rust has no `null` value, so the `dcbor` library provides a `CBOR::null()` method that returns a `CBOR` instance representing the `null` value.

And since all three elements of the array are being converted directly into CBOR, there is no problem constructing the heterogenous array.

> **✅ NOTE:** Of course, dCBOR doesn't support CBOR `undefined` or any of the other simple values, so the `dcbor` API doesn't have ways to let you construct them!

## Extracting from a Heterogenous Array

So now that we've gotten ourselves into this situation, how do we get the values back out? The `dcbor` library provides a set of methods for testing and extracting the CBOR major types, as well as unique values like `true`, `false`, and `null`.

In the example below we first begin by extracting our CBOR array from the composed `CBOR` instance. We then demonstrate several methods to either extract values or test them against expected values.

```rust
{{#rustdoc_include ../../tests/test_1.rs:test_10}}
```

## Maps

As long as all the types contained in a Rust `HashMap` or `BTreeMap` are supported by CBOR (we'll discuss how to make your own types CBOR-compatible in a later chapter), then converting them to CBOR and back is straightforward.

In the example below we round-trip a Rust `HashMap` with `String` keys and `Vec<String>` values all the way to serialized CBOR data and back again:

```rust
{{#rustdoc_include ../../tests/test_1.rs:test_11}}
```

Those familiar with JSON know that it only supports string keys, but CBOR supports any type of CBOR value as a key, and it's a common pattern to use integers as keys, which are much more compact:

```rust
{{#rustdoc_include ../../tests/test_1.rs:test_12}}
```

Note the use of `diagnostic_flat()` in this example, which returns the diagnostic notation with no line breaks or indentation. In previous examples we also used either `hex()` or `hex_annotated()` depending on the desired formatting.

> 🚧 **Work in Progress:** _More in this chapter and more chapters forthcoming!_
