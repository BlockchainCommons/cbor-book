# dCBOR Tags

As discussed in [Part I: CBOR Tags](../part_1/cbor_tags.md), CBOR tags are an integer that "tags" the CBOR data item that follows, specifying its type or meaning. This is a powerful feature of CBOR.

Let's say we wanted to define a tag that identifies a string as holding an ISO 4217 currency code like `USD` or `EUR`. We could just use a bare string, but if we want our type to be completely self-describing, we can define a tag for it.

As long as you are the only one using that tag, you can choose any integer you want. But if you want your structure to interoperate with other systems, you should use a tag that is registered with IANA, discussed previously [here](../part_1/cbor_tags.md#how-to-register-your-own-fcfs-tags).

For our demonstration we'll use the tag `33000`, which as of this writing is unassigned by IANA.

So how would we tag a string as a currency type? Let's start by defining a constant for our tag:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_1}}
```

We now associate our string with the tag by using the `to_tagged_value()` method:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_2}}
```

We can extract the tag and the tagged value using `try_into_tagged_value()`. The return type is a tuple of a `Tag` and the tagged item:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_3}}
```

The reason you have to call `value()` on the returned `Tag` to get back the numeric value is that a `Tag` may also include a human-readable name you can define for your tag. We'll discuss naming tags more later in this chapter.

> ✅ **NOTE:** A _tagged value_ is the combination of a tag and the value (data item) it tags. But the _value of the tag_ is the integer that identifies the tag.

If we print the diagnostic notation of our tagged value, we can see the tag in the output:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_4}}
```

As shown above, we can always extract the `(Tag, CBOR)` tuple from a tagged value, and then compare the tag value to our constant to see whether we want to process it further. But it's a common pattern to expect to find a specific tag in a particular place in a CBOR structure. So `dcbor` provides a convenience method `try_into_expected_tagged_value()` to test the tag value and return an error if it doesn't match. If it succeeds, it returns the tagged value for further processing.

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_5}}
```

## Tagging a Complex Structure

Let's say we want to combine our tagged currency code with an amount. Currency amounts can be tricky, because they are expressed as having decimal fractions, but many common floating point values, like `1.1` cannot be represented exactly in binary floating point, meaning that even highly-precise types like `f64` can't represent common currency values accurately.

So let's define a new type called `DecimalFraction` that holds an integer mantissa and a signed exponent. The exponent is the number of decimal places, so `1.1` would be represented as a mantissa of `11` with an exponent of `-1`, and `1.01` would be represented as a mantissa of `101` with an exponent of `-2`:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_6}}
```

> ✅ **NOTE:** We're not showing a lot of the typical boilerplate code here, like the `impl`s for `Debug`, `Clone`, `Display`, and things like `new()` methods. You can find the complete code in the repo for this book.

It turns out that [RFC8949 §3.4.4](https://www.rfc-editor.org/rfc/rfc8949.html#name-decimal-fractions-and-bigfl) already defines a CBOR structure for decimal fractions, so we can use that: it's just a two-element array with the exponent first and the mantissa second. It also reserves the tag `4` for decimal fractions, so we can use that as our tag.

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_7}}
```

So now we can create a `DecimalFraction` and convert it to CBOR, showing the diagnostic notation:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_8}}
```

Because conversion from CBOR to a given type can fail, we implement the `TryFrom<CBOR>` trait for our `DecimalFraction` type:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_9}}
```

Now we can round-trip our tagged value, converting it to CBOR and back to a `DecimalFraction`:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_10}}
```

## Implementing a Tagged String

We used a tagged string for our currency code, but we can also define a `CurrencyCode` type using the _newtype_ pattern. This is a common Rust idiom for creating a new type that wraps an existing type, like `String`, and provides additional functionality. In this case, the additional functionality is to implement `From<CurrencyCode> for CBOR` and `TryFrom<CBOR> for CurrencyCode`.

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_11}}
```

Now we can round-trip our `CurrencyCode` the same way we did with `DecimalFraction`:

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_12}}
```

## Combining the Two Types

Originally we set out to create a structure that combined a currency code with a decimal fraction: `CurrencyAmount`. We'd also like this structure to have it's own tag, so we'll use `33001`, which is also unassigned by IANA as of this writing.

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_13}}
```

Now that we have completely reusable constituents, we can define `CurrencyAmount` as a type that consists of a `CurrencyCode` and a `DecimalFraction`.

```rust
{{#rustdoc_include ../../tests/dcbor_tags.rs:example_14}}
```

Notice that in the above example, we're able to call the `to_cbor()` method on the `CurrencyCode` and `DecimalFraction` types, because the `dcbor` library includes a _blanket implementation_ for another trait called `CBOREncodable`, which automatically applies to any type that implements `Into<CBOR>` and `Clone`. (We implemented `From<CurrencyCode> for CBOR` and `From<DecimalFraction> for CBOR` which also implicitly implement the `Into<CBOR>` trait, so we get the `CBOREncodable` trait for free.)

The `CBOREncodable` trait gives us the `to_cbor()` method, which can be called on a `&self` (reference to self) unlike the `into()` method, which consumes the value. It also gives us the `to_cbor_data()` method, which returns the final, serialized CBOR data as a `Vec<u8>`.

> 🚧 **Work in Progress:** _More in this chapter and more chapters forthcoming!_
