# Determinism with dCBOR

> 🚧 **Work in Progress:** _This chapter is not yet ready for review!_

> "Be conservative in what you send, be liberal in what you accept."\
> — Postel's Law

John Postel was one of the architects of the early Internet, and his "law" also known as _The Robustness Principle_, suggests that when designing protocols or systems, you should be strict about the data you produce (i.e., follow the specifications closely) but lenient about the data you accept (i.e., be forgiving of minor deviations from the specifications).

Applying Postel's Law worked especially well— initially— in the context of hand-written HTML, allowing authors to create web pages that would work in most browsers, even if they were not strictly valid HTML. The idea is that by being lenient, you can accommodate a wider range of inputs and make your system more robust. However, this led to HTML parsers becoming more complex and less predictable, as they had to handle a wide variety of malformed HTML.

So it turns out that Postel's Law is a double-edged sword: while it fosters compatibility, it can also mask errors, lead to insecure systems, or encourage sloppy implementations when abused. Recognition of this led to the development of specifications like XHTML, which must validate as well-formed XML, and the HTML5 specification, which is much more explicit about what is allowed and what is not.


So, you're using dCBOR to encode and decode your data: _determinism achieved!_

Well, not really. Determinism as a goal is not a simple problem, and no semantic layer can guarantee it by itself. Technically the top layer, the application, could specify every aspect of a deterministic encoding, but by making choices about the lower layers, you can reduce the cognitive load on the application designers and implementers.

dCBOR identifies and avoids a common set of "foot-guns" that can lead to non-deterministic encodings, so by adopting it you gain those benefits. But when determinism is a goal, you also have to design your data structures with determinism in mind, which is why we visit this topic several times in this book.

> ✅ **Note:** _Not everyone needs determinism._ You might adopt CBOR or even the `dCBOR` library for other reasons, including its support for Gordian Envelope, which we'll discuss in Part III. Envelope has its own ways of supporting determinism, and its own set of best practices for achieving it.

## Optionality

In CBOR there are three common ways to represent “no value” for an otherwise‐required field:

Representation	Example diagnostic	Possible semantic meaning
Omitted key	{name:"Wolf"}	“We do not track this attribute.”
Null value	{name:null}	“We track it but it is unset.”
Empty string	{name:""}	“User explicitly refused to provide it.”

Unless a profile dictates which of the three is authoritative, two encoders can serialize the same high‑level state differently.

```cddl
; under‑specified
profile = {
  name : null / tstr,        ; absent also legal
}
```

Diagnostic examples that all mean “name is unknown”:

```cbor
{name:null}
{}                            ; key omitted
{name:""}                     ; empty string
```

Canonical resolution

Decide a single rule and reject the others.  Example: omit the field when unknown.

```cddl
profile = {
  ? name : tstr               ; present → non‑empty text only
}
```

Canonical encoder:

```cbor
{}                            ; unknown
{name:"Wolf"}                ; known
```

Decoder rejects {name:null} and {name:""}.


## Type Choice

One concept, many CBOR base types or tags.

```cddl
; under‑specified
timestamp = #6.0(tstr) / #6.1(int)
```

Same instant, two encodings:

```cbor
0("2025-05-07T10:00:00Z")
1(1746621600)
```

Canonical rule – “always Tag 1”.

```cddl
timestamp = #6.1(int)
```

Canonical encoding:

```cbor
1(1746621600)
```


## Numeric Normalization

Multiple mathematically equivalent encodings after the type is fixed.

```cddl
; under‑specified decimal
amount = #6.4([exponent:int, mantissa:int])
```

All three mean “one hundred”:

```cbor
4([2,1])    ; [2,1]
4([1,10])   ; [1,10]
4([0,100])  ; [0,100]
```

Canonical rule – “mantissa has no trailing zeroes, exponent minimal”.

```cddl
amount = #6.4([0, uint])   ; exponent always 0
```

Canonical encoding:

```cbor
4([0,100])
```

## Structural Modeling

Different container shapes for the same record.

```cddl
; under‑specified
coord = {lat:float,lon:float}
      / [lat:float,lon:float]
```

Equal positions:

```cbor
{lat:36.0,lon:-115.0}
[36.0,-115.0]
```

Canonical rule – “ordered array only”.

```cddl
coord = [lat:float, lon:float]
```

Canonical encoding:

```cbor
[36.0,-115.0]
```

## Redundancy & Aliasing

Synonymous fields or duplicate information.

```cddl
; under‑specified
distance = {
  (meters / kilometres) : float
}
```

Two encodings for the same five‑meter length:

```cbor
{meters:5.0}
{kilometres:0.005}
```

Canonical rule – “meters only”.

```cddl
distance = {meters:float}
```

Canonical encoding:

```cbor
{meters:5.0}
```

## Precision & Quantization

Choice of width or scaling.

```cddl
; under‑specified
price = float          ; 19.99 could be any float width
```

```cbor
fa4198f5c29            ; float32 19.99
fb4033f5c28f5c29f      ; float64 19.99
```

Canonical rule – “store cents as unsigned integer”.

```cddl
price = uint           ; 19.99 USD → 1999
```

Canonical encoding:

```cbor
1999
```

## Unit & Scale Variation

Same quantity in different units.

```cddl
; under‑specified
duration = {seconds:uint} / {milliseconds:uint}
```

Two encodings for three seconds:

```cbor
{seconds:3}
{milliseconds:3000}
```

Canonical rule – “microseconds only”.

```cddl
duration = {microseconds:uint}
```

Canonical encoding:

```cbor
{microseconds:3000000}
```

## Tagging Conventions

Optional tags or competing tags.

```cddl
; under‑specified
blob = bytes / #6.23(tstr)      ; raw vs Base64
```

Two encodings for the same bytes:

```cbor
h'DEADBEEF'
23(h'DEADBEEF')
```

Canonical rule – “raw bytes, never Tag 23”.

```cddl
blob = bytes
```

Canonical encoding:

```cbor
h'DEADBEEF'
```

## Extension Points & Unknowns

Open maps that tolerate extra keys.

```cddl
; under‑specified
config = { * tstr => any }
```

```cbor
{version:1,foo:7}
{version:1,bar:"x"}
```

Canonical rule – closed world with an allow‑list.

```cddl
config = {
  version : uint,
  params  : {
    * ("foo" / "bar" / "baz") => int
  }
}
```

Only allowed encoding for version 1 and foo = 7:

```cbor
{version:1,params:{foo:7}}
```

> 🚧 **Work in Progress:** _More in this chapter and more chapters forthcoming!_
