# CBOR Schemas with CDDL

## Defining Structure Amidst Flexibility

In previous chapters, we explored the fundamental mechanics of CBOR encoding, learning how basic data types like integers, strings, arrays, and maps are represented in binary. We saw how CBOR's structure, based on Major Types and Additional Information, allows for a self-describing format that is efficient, especially in constrained environments. However, while CBOR itself defines _how_ individual data items are encoded, it doesn't inherently restrict the _overall structure_ of the data exchanged between systems. An application might expect a map with specific keys and value types, or an array containing a precise sequence of elements. Without a way to formally define these expectations, interoperability relies solely on human-readable documentation and the diligence of implementers – a scenario prone to errors and ambiguity.

While CBOR itself is a schemaless and self-describing format, there are many times a formal schema can be helpful to define the structure of the data being exchanged. This is especially true in cases where the data is complex, or when multiple systems need to interoperate. A schema can help ensure that all parties agree on the expected structure and types of data, reducing the risk of errors and misunderstandings.

This is where the Concise Data Definition Language (CDDL) comes in. Standardized by the IETF in [RFC-8610](https://datatracker.ietf.org/doc/html/rfc8610) (and updated by [RFC-9682](https://datatracker.ietf.org/doc/html/rfc9682)), CDDL provides a formal, unambiguous, and human-readable notation specifically designed to express the structure of data formats that use CBOR (and, conveniently, JSON, due to its data model being a subset of CBOR's). Its primary goal is to make defining protocol messages and data formats easy and clear.  

Having understood _how_ individual CBOR items are built, we now turn to specifying _what_ structure those items should collectively form. This chapter introduces the essentials of CDDL, focusing on equipping engineers with the practical knowledge needed to define and understand CBOR schemas. We will cover:

- The core concepts and syntax of CDDL.
- How to represent standard CBOR types and literal values.
- Defining arrays, maps, and the crucial concept of CDDL _groups_ for sequences.
- Using operators to control occurrences, choices, and value constraints.
- Building complex schemas by composing simpler, reusable rules.
- The role of CDDL in data validation and the tooling ecosystem.

A key focus will be understanding how CDDL models sequences of CBOR items using _groups_, a concept distinct from CBOR arrays or maps, and how this directly relates to the sequential nature of CBOR encoding. By the end of this chapter, you should be able to read and write basic CDDL schemas to define the structure of your CBOR data, laying the foundation for more robust and interoperable systems. We will prioritize practical application over exhaustive coverage of every CDDL feature, aiming for a solid working understanding rather than covering every detail of the full specification.

## Validating CDDL Interactively

Before diving into the details of CDDL, it's helpful to have a way to validate and experiment with CDDL schemas interactively. [cddl.anweiss.tech](https://cddl.anweiss.tech/) provides a convenient online tool for this purpose.

## Core Concepts: Rules, Assignments, and Types

CDDL achieves its goal of unambiguous structure definition through a relatively simple grammar, inspired by Augmented Backus-Naur Form (ABNF) but tailored for CBOR/JSON data models. At its heart, a CDDL specification consists of one or more _rules_.  

### Rules and Assignments

A rule defines a name for a specific data structure or type constraint. The most basic assignment operator is `=`, which assigns a name (the _rule name_) on the left to a _type definition_ on the right. Rule names are typically lowercase identifiers, potentially containing hyphens or underscores.

CDDL:
```cddl
; This is a comment. Comments start with a semicolon and run to end-of-line.

my-first-rule = int  ; Assigns the name "my-first-rule" to the CBOR integer type.

device_id = uint     ; Assigns the name "device_id" to the CBOR unsigned integer type.
```

CDDL is whitespace-insensitive, except within literal strings. Comments are essential for documenting the schema and explaining the intent behind rules.

Besides the basic assignment `=`, CDDL provides two other assignment operators for extending existing rules:

- `/=` Appends alternative choices to an existing rule. So a specification for an integer _or_ a text string can be defined as:

CDDL:
```cddl
my-rule = int / tstr
```

or alternatively:

CDDL:
```cddl
my-rule = int
my-rule /= tstr
```

- `//=` Appends alternative group choices to an existing rule. This is used for adding choices between sequences, which we'll explore when discussing groups.

### The Prelude: Standard Definitions

Every CDDL specification implicitly includes a set of predefined rules known as the _prelude_. This prelude defines convenient names for common CBOR types and some basic constraints. You don't need to define these yourself; they are always available:  

| Category    |   Name      |   Description                                   |
|-------------|-------------|-------------------------------------------------|
| Basic Types | `bool`      | Boolean value                                   |
|             | `uint`      | Unsigned integer (technically, a non-negative integer) |
|             | `nint`      | Negative integer                                |
|             | `int`       | Unsigned or negative integer                    |
|             | `float16`   | 16-bit floating point                           |
|             | `float32`   | 32-bit floating point                           |
|             | `float64`   | 64-bit floating point                           |
|             | `float`     | Any floating point                              |
|             | `bstr`      | Byte string                                     |
|             | `tstr`      | Text string                                     |
|             | `any`       | Any single CBOR data item                       |
| Constants   | `null`      | Null value                                      |
|             | `true`      | Boolean true                                    |
|             | `false`     | Boolean false                                   |
|             | `undefined` | Undefined value                                 |
| Aliases     | `nil`       | Alias for `null`                                |
|             | `bytes`     | Alias for `bstr`                                |
|             | `text`      | Alias for `tstr`                                |

These prelude types form the building blocks for more complex definitions. For instance, instead of just saying `int`, you can often be more specific using `uint` or `nint` if the sign is known. Alternatively, a way to think about the definition of `int` is that it is a union of `uint` and `nint`, but the prelude provides a more convenient shorthand:

CDDL:
```cddl
int = uint / nint
float = float16 / float32 / float64
```

## Representing Basic CBOR Types and Literals

CDDL provides direct ways to refer to the fundamental CBOR data types, largely leveraging the names defined in the prelude. It also allows specifying literal values that must appear exactly as written.

### Standard Types

As seen above, the prelude provides names for most standard CBOR types:

- **Integers**: `uint`, `nint`, `int`.
- **Floating-Point**: `float16`, `float32`, `float64`, `float`. These correspond to the IEEE 754 half-, single-, and double-precision formats supported by CBOR's Major Type 7.
- **Simple Values**: `bool`, `true`, `false`, `null`, `undefined`. These map directly to the specific simple values in Major Type 7.
- **Strings**: `tstr` (UTF-8 text string, Major Type 3), `bstr` (byte string, Major Type 2).
- **Catch-all**: `any` represents any single, well-formed CBOR data item.

CDDL:
```cddl
; Examples using standard types
message-counter = uint
temperature = float
is-active = bool
user-name = tstr
raw-payload = bstr
any-value = any ; Allows any CBOR item including `null`.
```

### Literal Values

CDDL allows you to specify that a data item must be a specific literal value:

- **Integers**: `10`, `0`, `-1`, `42`.
- **Floats**: `1.5`, `-0.0`, `3.14159`.
- **Text Strings**: `"hello"`, `""`, `"a specific key"`. Text strings are enclosed in double quotes. Escaping rules similar to JSON apply within the CDDL source (e.g., `"\"quoted\""`) but the resulting CBOR string itself contains the literal characters without CDDL escapes.
- **Byte Strings**: `h'010203'`, `h''`. Byte strings are represented using hexadecimal notation prefixed with `h` and enclosed in single quotes.

CDDL:
```cddl
; Examples using literal values
message-type = 1                 ; The value must be the integer 1
protocol-version = "1.0"         ; The value must be the text string "1.0"
fixed-header = h'cafef00d'       ; The value must be these specific four bytes
status-code = 200 / 404 / 500    ; The value must be one of these integers
```

Literal values are often used as discriminators in choices or as fixed keys in maps.

## Defining Collections: Arrays, Maps, and Groups

Beyond simple scalar types, CDDL provides syntax for defining the structure of CBOR arrays (Major Type 4) and maps (Major Type 5). Crucially, it also introduces the concept of _groups_ delimited by parentheses (`()`) to define sequences of items that are _not_ enclosed within a CBOR array or map structure. Understanding the distinction between these is vital for correctly modeling CBOR data.

### Arrays (`[]`)

CDDL uses square brackets `[]` to define CBOR arrays. Inside the brackets, you specify the type(s) of the elements that the array should contain.

CDDL:
```cddl
; An array containing exactly three unsigned integers
triplet = [uint, uint, uint]

; An array containing a text string followed by any CBOR item
labelled-item = [tstr, any]

; An empty array
empty-array = []

; An array where the first element is a boolean, and the second is either an int or null
mixed-array = [bool, int / null]
```

Occurrence indicators (covered later) can be used to specify variable numbers of elements. The definition within the brackets describes the sequence of CBOR items expected _within_ the CBOR array structure itself.

### Maps (`{}`)

CDDL uses curly braces `{}` to define CBOR maps. Inside the braces, you define the expected key-value pairs. A key difference from JSON is that CBOR map keys can be _any_ CBOR data type, not just strings. CDDL reflects this flexibility.  

There are two primary ways to specify map members:

1. **`key: type`**: This form requires the key to be a literal `tstr` or `int`, or a `tstr` or `int` type that has a single literal value constraint. It's a shorthand commonly used when keys are simple strings or integers.
2. **`keytype => valuetype`**: This is the more general form. `keytype` can be any CDDL type definition (e.g., `tstr`, `uint`, `my-custom-rule`, a literal value), and `valuetype` defines the type of the corresponding value.

CDDL:
```cddl
; A map with specific string keys
simple-object = {
  "name": tstr,
  "age": uint,
  is-verified: bool  ; Bare words are shorthand for "is-verified": bool
}

; A map using integer keys and the => syntax
indexed-data = {
  1 => tstr,         ; Key is integer 1, value is text string
  2 => bstr,         ; Key is integer 2, value is byte string
 ? 3 => float       ; Key 3 is optional (using '?')
}

; A map where keys must be unsigned integers and values are text strings
; The '*' indicates zero or more occurrences of this key/value pattern
lookup-table = {
  * uint => tstr
}

; An empty map
empty-map = {}
```

**Important Considerations for Maps:**

- **Key Types**: Remember that CBOR allows non-string keys. CDDL fully supports defining maps with integer, byte string, or even complex keys.

- **Order**: Although key-value pairs must be serialized in _some_ order in the CBOR encoding, CDDL map definitions, like CBOR maps themselves, are generally considered _orderless_. Validation typically checks for the presence of required keys and type correctness, not the specific order in the encoded bytes. Deterministic encoding profiles, discussed later in this book, impose strict ordering rules.  

- **Uniqueness**: The core CBOR specification doesn't strictly require map keys to be unique. However, most applications assume unique keys, and CDDL validation tools often enforce uniqueness by default or provide options to control this behavior. Relying on duplicate keys is generally discouraged.

### Groups (`()`) - Defining Sequences

Perhaps the most distinctive structural element in CDDL compared to JSON-centric schema languages is the _group_, denoted by parentheses `()`. A group defines an ordered sequence of one or more CBOR data items _without_ implying an enclosing CBOR array or map structure.

This concept directly mirrors how CBOR works: data items are encoded sequentially one after another. A group in CDDL allows you to name and constrain such a sequence.

CDDL:
```cddl
; A group containing an unsigned integer followed by a text string
record-header = (uint, tstr)

; A group containing two floats
point-2d = (float, float)
```

At first glance, `point-2d = (float, float)` might look similar to `point-array = [float, float]`. However, they define fundamentally different structures:

- `point-array` defines a CBOR **array** (e.g., `[1.0, 2.5]`, encoded starting with `0x82`) containing two floats.
- `point-2d` defines a **sequence** of two CBOR floats (e.g., `1.0` followed by `2.5`, encoded as `0xf93c00` followed by `0xfa40200000`, assuming preferred serialization).

**Why are groups useful?**

1. **Partial Arrays**: Groups can be used to define partial arrays or sequences of items without needing to wrap them in a sub-array structure. For example, a `key` type that is an array of three items, where the first is a text string and the second and third are byte strings, could be defined as:

CDDL:
```cddl
key = [key-info, bstr, bstr]
key-info = tstr
```

But if the two byte strings are the key's conceptual "body", how would we use CDDL to make that clear? Using groups! The following definition is equivalent to the above, but it makes the relationship between the key and its body clear:

CDDL:
```cddl
key = [key-info, key-body]
key-info = tstr
key-body = (bstr, bstr)
```

2. **Structuring Map Members**: Groups can structure related members within a map without requiring a nested map.

CDDL:
```cddl
person = {
    name: tstr,
    address
}

; Group defines the address structure
address = (
    street: tstr,
    city: tstr,
    zip: uint
)
```

This defines that the `street`, `city`, and `zip` keys are logically related and should appear (conceptually) together, but they remain direct members of the `person` map, the above definition being equivalent to:

CDDL:
```cddl
person = {
    name: tstr,
    street: tstr,
    city: tstr,
    zip: uint
}
```

This makes `address` a reusable group that can be referenced in multiple places, enhancing modularity and readability.

3. **Defining Choices Between Sequences (Group Choice `//`)**: Allows choosing between different sequences of items.

CDDL:
```cddl
message = {
    header,
    payload // error-report
}
header = (version: uint, msg_id: uint)
payload = (data: bstr)
error-report = (code: int, reason: tstr)
```

Groups are powerful because they leverage the fundamental sequential nature of CBOR encoding. While JSON schema languages might struggle to represent bare sequences outside of arrays, CDDL embraces them, providing a precise way to model structures common in binary protocols where items follow each other without explicit delimiters.

### Defining Cardinality

CDDL provides several operators to control the cardinality of elements within arrays, maps, and groups. These operators specify how many times a given type or group can occur in a sequence.

- `?`: **Optional** (zero or one time).

CDDL:
```cddl
optional-id = [ ?uint ]      ; Array with 0 or 1 uint
config = ( tstr, ?bool )     ; Group: tstr, optionally followed by bool
```

- `*`: **Zero or more** times.

CDDL:
```cddl
int-list = [ *int ]          ; Array with any number of ints (including zero)
byte-chunks = ( *bstr )      ; Group: sequence of zero or more byte strings
```

- `+`: **One or more** times.

CDDL:
```cddl
non-empty-list = [ +tstr ]   ; Array with at least one text string
data-record = ( uint, +float ) ; Group: uint followed by one or more floats
```

- `n*m`: **Specific range** (`n` to `m` times, inclusive). If `n` is omitted, it defaults to 0. If `m` is omitted, it defaults to infinity.

CDDL:
```cddl
rgb-color = [ 3*3 uint ]      ; Array with exactly 3 uints
short-ids = [ 1*5 int ]       ; Array with 1 to 5 ints
max-10-items = [ *10 any ]    ; Array with 0 to 10 items of any type
at-least-2 = [ 2* bstr ]      ; Array with 2 or more byte strings
```

These indicators provide fine-grained control over the cardinality of elements within sequences and arrays.

### Choices

CDDL offers two ways to define alternatives:

1. **Type Choice (`/`)**: Allows choosing between different _types_ for a single data item slot.

CDDL:
```cddl
identifier = tstr / uint       ; An identifier is either a text string or a uint
config-value = bool / int / tstr / null
measurement = [ tstr, int / float ] ; Array: string followed by an int OR a float
```

2. **Group Choice (`//`)**: Allows choosing between different _groups_ (sequences) of items. This is used when the choice affects multiple items or map members.

CDDL:
```cddl
contact-method = {
    (email: tstr) //
    (phone: tstr) //
    postal-address
}
postal-address = (street: tstr, city: tstr)

response = {
    (status: 200, body: bstr) // (status: 500, error: tstr)
}
```

In the `response` example, the choice affects both the `status` value and the subsequent item (`body` or `error`).

### Value Constraints and Control Operators

Beyond type and occurrence, CDDL allows constraints on the actual values or properties of data items using literal value ranges and _control operators_ (often called "dot operators"). Control operators act as extensions to the core grammar, providing hooks for more sophisticated validation. The prelude defines several useful ones, and others can be defined by specific CDDL profiles or applications.  

- **Ranges (`..`)**: Defines an inclusive range for numerical types or literal values.

CDDL:
```cddl
age = uint .le 120             ; Using prelude.le (less than or equal)
percentage = 0..100            ; Value must be int between 0 and 100 inclusive
temperature = -40..50          ; Value must be int between -40 and 50
http-status-ok = 200..299      ; Integer range for successful HTTP status
first-byte = 0x00..0xFF        ; Integer range using hex literals
```

Range checks can also be combined with prelude operators like `.lt` (less than), `.le` (less than or equal), `.gt` (greater than), `.ge` (greater than or equal), `.eq` (equal), `.ne` (not equal).

- **Common Control Operators**:

- `.size uint` / `.size (min..max)`: Constrains the size (length). For `bstr` and `tstr`, it's the number of bytes. For arrays, it's the number of elements. For maps, it's the number of key-value pairs.

CDDL:
```cddl
short-string = tstr .size (1..64)
sha256-hash = bstr .size 32
coordinate = [ float ] .size 2  ; Array must have exactly 2 floats
simple-map = { * tstr => any } .size (1..5) ; Map with 1 to 5 pairs
```

> **⚠️ NOTE**: The whitespace before "dot operator" is significant. If you get errors, check for missing whitespace.

- `.regexp tstr`: Validates that a `tstr` matches a given regular expression pattern (syntax follows XML Schema Definition Language (XSD) style regular expressions, as per [XSD Appendix F](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/#regexs)).

CDDL:
```cddl
email = tstr .regexp "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"
iso-date = tstr .regexp "\d{4}-\d{2}-\d{2}"
```

- `.cbor type` / `.cborseq type`: Validates that a `bstr` contains bytes that are a valid CBOR encoding of the specified `type` or a sequence (`.cborseq`) of items matching `type`. This is useful for embedding CBOR within CBOR.

CDDL:
```cddl
signed-data = {
    payload: bstr .cbor any, ; Payload is bytes that decode to some CBOR item
    signature: bstr
}
message-stream = bstr .cborseq log-entry ; Bytes contain a sequence of log entries
log-entry = [timestamp, tstr] ; Assuming log-entry is defined elsewhere
timestamp = uint
```

These operators allow schema authors to declaratively state constraints without needing to specify the validation logic itself. CDDL tools interpret these declarations to perform the checks.  

> **✅ TIP:** dCBOR, which we will discuss later in this book, also defines two additional operators, `.dcbor` and `.dcborseq`, which are exactly like `.cbor` and `.cborseq` except that they also require the encoded data item(s) be valid dCBOR.

The following table summarizes the most frequently used operators for controlling structure and content:

|Operator|Name|Meaning|Example Usage|
|---|---|---|---|
|`?`|Optional|Zero or one occurrence|`? int`|
|`*`|Zero or More|Zero or more occurrences|`* tstr`|
|`+`|One or More|One or more occurrences|`+ bool`|
|`n*m`|Range Occurrence|n to m occurrences|`2*4 float`|
|`/`|Type Choice|Choose between listed types|`int / tstr`|
|`//`|Group Choice|Choose between listed groups|`(int) // (tstr, bool)`|
|`..`|Value Range|Value within numerical range|`0..100`|
|`.size`|Size Control|Constrain byte/element/pair count|`tstr.size (1..10)`|
|`.regexp`|Regex Control|Match text string pattern|`tstr.regexp "..."`|
|`.cbor`|Embedded CBOR|Byte string is valid CBOR of type|`bstr.cbor my_type`|
|`.cborseq`|Embedded CBOR Sequence|Byte string is valid CBOR sequence|`bstr.cborseq my_type`|

## Building and Reusing Definitions

While the basic types and operators are powerful, the real strength of CDDL for defining complex data structures lies in its ability to build definitions compositionally by referencing other rules.

### Rule Referencing

Once a rule is defined with a name, that name can be used anywhere a type definition is expected in another rule. This allows breaking down complex structures into smaller, manageable, named components.

CDDL:
```cddl
; Define a structure for a person's name
name-structure = {
  first: tstr .size (1..50),
  last: tstr .size (1..50),
  ? middle: tstr .size (1..50) ; Optional middle name
}

; Define contact information choices
contact-info = email-address / phone-number

email-address = tstr .regexp "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"
phone-number = tstr .regexp "\+?[1-9]\d{1,14}" ; Simple E.164 regex example

; Define the main person structure, referencing other rules
person = {
  name: name-structure,       ; Use the name-structure rule here
  age: uint .le 120,          ; Updated to include whitespace before .le
  ? contact: contact-info     ; Optional contact info, using the choice rule
}
```

In this example, the `person` rule is defined using references to `name-structure` and `contact-info`. This makes the `person` definition concise and readable. If the structure of a name or contact information needs to change, the modification only needs to happen in one place (the `name-structure` or `contact-info` rules), improving maintainability.

### Modularity and Readability

This compositional approach is key to managing complexity in large data format specifications. By breaking down the overall structure into logical, named sub-components (rules and groups), CDDL schemas become:

- **More Readable**: Each rule focuses on a specific part of the data structure.
- **More Maintainable**: Changes to a shared structure are localized.
- **More Reusable**: Common structures (like timestamps, identifiers, addresses) can be defined once and referenced wherever needed.

This mirrors good software engineering practices, applying principles of modularity and abstraction to data definition. This compositional design aids in creating the unambiguous descriptions that are a primary goal of CDDL.  

### Practical Example: The "Gadget" Revisited

Let's revisit the nested JSON/CBOR example from a previous chapter and define its structure using CDDL:

**JSON/CBOR Diagnostic:**

```
{
  "name": "Gadget",
  "id": 12345,
  "enabled": true,
  "parts": [ "bolt", "nut" ],
  "spec": { "size": 10.5, "data": h'010000ff' }
}
```

**CDDL Definition:**

CDDL:
```cddl
; Define the top-level type for validation (often the first rule)
top-level = gadget

; Define the main gadget structure
gadget = {
  "name": tstr .size 1.., ; Name must be at least 1 byte
  "id": uint,
  "enabled": bool,
  "parts": [ + tstr ],      ; Array of one or more text strings
  "spec": gadget-spec       ; Reference the gadget-spec rule
}

; Define the structure for the specification sub-object
gadget-spec = {
  "size": float,       ; Allows float16, float32, or float64
  "data": bstr         ; The raw binary data
}
```

This CDDL schema precisely defines the expected structure:

- A map (`gadget`) with five required keys: `"name"`, `"id"`, `"enabled"`, `"parts"`, and `"spec"`.
- `"name"` must be a non-empty text string.
- `"id"` must be an unsigned integer.
- `"enabled"` must be a boolean.
- `"parts"` must be an array containing one or more text strings.
- `"spec"` must be a map conforming to the `gadget-spec` rule.
- The `gadget-spec` map requires keys `"size"` (a float) and `"data"` (a byte string).

Notice how the CDDL directly defines the `data` field as `bstr`, reflecting CBOR's native handling of binary data, unlike the base64 encoding necessary in the JSON representation. This schema clearly communicates the expected format for any system processing "gadget" data.

## Validation and the Tooling Ecosystem

Defining a schema is only part of the story. A major practical benefit of using CDDL is the ability to automatically validate CBOR data against a schema.  

### The Concept of Validation

Validation is the process of checking whether a given CBOR data instance conforms to the rules specified in a CDDL schema. Conceptually, a CDDL validator tool takes two inputs:

1. The CDDL schema definition (e.g., a `.cddl` file).
2. The CBOR data instance (usually as raw bytes).

The validator then processes the CBOR data according to the rules defined in the schema, starting from a designated root rule (often the first rule in the file, or explicitly specified). It outputs whether the data is valid according to the schema, often providing details about any discrepancies if validation fails.  

### Benefits of Validation

Automated validation provides significant benefits:

- **Error Detection**: Catch malformed data early, whether from external sources or internal bugs.
- **Interoperability**: Ensure that systems exchanging CBOR data adhere to the agreed-upon structure.
- **API Contract Enforcement**: Use CDDL schemas as machine-readable contracts for APIs that consume or produce CBOR.
- **Security**: Validate that incoming data conforms to expected structural constraints, preventing certain classes of injection or processing errors. While not a substitute for comprehensive security analysis, structural validation is a valuable defense layer.  


### Tooling Ecosystem

A growing ecosystem of tools and libraries supports working with CDDL. While this book won't provide tutorials for specific tools, it's important to be aware of their existence:  

- **Implementations**: Libraries for parsing CDDL and validating CBOR/JSON data are available in various languages, including Rust (`cddl-rs` ), Node.js (`cddl`), and potentially others like Python, Go, or Java (e.g., via wrappers like `cddl2java` mentioned in ).  

- **Functionality**: Common features include:
    - Parsing CDDL schemas into an Abstract Syntax Tree (AST).  
    - Validating CBOR data against a CDDL schema.  
    - Validating JSON data against a CDDL schema.  
    - Checking CDDL syntax conformance.  
    - Some tools might offer experimental features like generating documentation or code stubs, though code generation is not a primary design goal of CDDL itself.  

- **Online Tools**: Resources like [cddl.anweiss.tech](https://cddl.anweiss.tech/) offer CDDL validation, allowing interactive experimentation.

The availability of these tools enables a _schema-driven development_ workflow. The CDDL schema can serve as a central artifact for documentation, automated testing (validation), runtime checks, and ensuring consistency across different parts of a system or between collaborating teams. This elevates CDDL from merely a descriptive language to an active component in building robust CBOR-based applications.  

## Conclusion: Laying the Schema Foundation

This chapter has introduced the Concise Data Definition Language (CDDL) as the standard way to define the structure of CBOR data. We've moved from understanding _how_ individual CBOR items are encoded to specifying _what_ overall structure those items should form in a given application or protocol.

We covered the core concepts: rules defined using assignments (`=`, `/=`, `//=`), the use of standard types from the prelude (`uint`, `tstr`, `bool`, etc.), and the specification of literal values. We explored how CDDL defines CBOR arrays (`[]`) and maps (`{}`), noting the flexibility of map keys in CBOR. Crucially, we delved into CDDL groups (`()`) and their role in defining sequences of items without explicit CBOR delimiters, highlighting how this feature directly maps to CBOR's sequential encoding and distinguishes CDDL from JSON-centric schema languages. We also learned how to control structure using occurrence indicators (`?`, `*`, `+`, `n*m`), define choices (`/`, `//`), and apply constraints using value ranges (`..`) and practical control operators like `.size`, `.regexp`, and `.cbor`. Finally, we saw how rule referencing enables modular, readable, and reusable schema design, and how the existence of validation tools makes CDDL a practical asset for development.

**Best Practices for Writing CDDL:**

As you start defining your own CBOR structures with CDDL, keep these practices in mind:

- **Clarity over Brevity**: Prioritize making the schema easy to understand. Use comments (`;`) liberally to explain intent and choices.
- **Meaningful Names**: Choose descriptive names for rules that reflect their purpose.
- **Modularity**: Break down complex structures into smaller, well-named rules. This improves readability, maintainability, and reuse.
- **Start Specific, Generalize Carefully**: Define the expected structure as precisely as possible initially. Use broad types like `any` or wide occurrence ranges (`*`) only when truly necessary, as overly permissive schemas offer less validation value.
- **Consider the CBOR Data Model**: Think about how your CDDL definition maps to the underlying CBOR types and encoding, especially regarding the distinction between groups (`()`) and container types like arrays (`[]`) and maps (`{}`).

With the fundamentals covered here, you are equipped to use CDDL to bring clarity and rigor to your CBOR-based data formats. This foundation is essential as we move forward to explore more advanced CBOR topics. CDDL schemas are instrumental in understanding and validating the structures used within CBOR Tags, ensuring the correctness of data before applying deterministic encoding rules (dCBOR), and understanding the precise layout of nested structures like Gordian Envelope.
