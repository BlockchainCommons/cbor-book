# Determinism: Why Consistent Encodings Matter

## 1.1 Introduction: The Illusion of Sameness

Consider a common scenario in software engineering: comparing two data structures that represent the same logical information. Perhaps they are configuration objects loaded from different sources, snapshots of system state taken at different times but believed to be identical, or messages exchanged between distributed components. A developer might reasonably expect that if these structures hold the same values—field for field, element for element—a simple byte-wise comparison of their serialized forms would confirm their equality. Yet, surprisingly often, this comparison fails. Two objects, logically identical, produce different sequences of bytes when serialized.

This discrepancy arises because many common data serialization formats, including text-based ones like JSON and even efficient binary formats like Protocol Buffers or CBOR itself, allow flexibility in how data is represented. The same logical map might have its keys ordered differently; the same number might be encoded with varying precision or length; the same string might have subtle variations in character encoding or normalization. While these variations are often _semantically irrelevant_ at the data model level, they result in distinct byte sequences.

This phenomenon, where serialization yields inconsistent byte outputs for logically equivalent inputs, can be deeply problematic. Processes downstream that rely on these byte representations—such as cryptographic hashing, digital signature verification, distributed consensus mechanisms, or simple data comparison—may behave unpredictably or fail entirely. This variability acts much like a hidden, uncontrolled input, introducing non-determinism into systems that are otherwise expected to be predictable, leading to bugs that are notoriously difficult to diagnose and fix, akin to issues stemming from uninitialized memory or thread race conditions. Understanding and controlling this variability through deterministic encoding is therefore not merely an academic exercise but a practical necessity for building robust, secure, and interoperable systems. This chapter explores the fundamental need for deterministic encoding, the challenges involved, and the landscape of previous efforts to achieve it.

## 1.2 Defining Deterministic Encoding

At its core, **Deterministic Encoding** is an encoding process designed to eliminate ambiguity. It employs specific rules and makes deliberate choices during serialization to ensure that logically equivalent inputs at the data model level _always_ produce the exact same sequence of encoded bytes. This is distinct from the general term **Serialization**, which simply refers to the process of representing data model items (like numbers, strings, arrays, maps) as encoded data items, potentially allowing for multiple valid representations.

The term **Canonicalization** is often used synonymously with deterministic encoding, emphasizing the goal of producing a single, standard, or "canonical" form for any given piece of data. Several systems aim for this canonical property, where the serialization guarantees byte consistency for the same in-memory data structure, regardless of the implementation or environment.

Within the CBOR ecosystem (RFC 8949), related concepts exist that represent steps towards reducing variability, though they don't necessarily guarantee full cross-implementation determinism on their own:

- **Preferred Serialization:** A recommendation aiming for the shortest possible encoding for a data item's head (the initial bytes indicating type and length/value), without expending extra effort like sorting map keys.
- **Basic Serialization:** Builds on Preferred Serialization by adding the constraint that indefinite-length encoding (where the total length isn't known upfront) must not be used for strings, arrays, or maps.

While Preferred and Basic Serialization reduce encoding variability, true Deterministic Encoding, such as [CBOR Common Deterministic Encoding (CDE)](https://datatracker.ietf.org/doc/draft-ietf-cbor-cde/), imposes stricter rules, like mandatory map key sorting, to achieve the goal of a unique byte sequence for equivalent data.

Understanding why these stricter rules are necessary requires examining the common sources of non-determinism in data serialization:

- **Map/Object Key Order:** In data models like JSON objects or CBOR maps, the order of key-value pairs is generally considered semantically insignificant. `{"name": "Alice", "id": 123}` is logically the same as `{"id": 123, "name": "Alice"}`. However, without a rule mandating a specific order (e.g., sorting keys alphabetically), serializers might output these pairs in different orders, leading to different byte sequences. This is a major source of non-determinism in formats like JSON, Protobuf, and basic CBOR. Deterministic schemes typically mandate sorting keys based on a well-defined comparison, such as lexicographical sorting of the UTF-16 key strings (as in [JCS](https://www.rfc-editor.org/rfc/rfc8785)) or byte-wise lexicographical sorting of the _encoded_ keys (as in CBOR CDE).

- **Number Representation:** Numbers can often be encoded in multiple ways:

    - _Integers:_ Small integers might fit into short forms, but longer encodings could technically be valid in some formats. Varint encodings (used in Protobuf) can sometimes represent the same number using different byte lengths, especially if leading zeros aren't strictly prohibited. CBOR's Preferred Serialization aims for the shortest form, but deterministic rules make this mandatory. Arbitrary-precision integers (bignums) also need clear rules to avoid ambiguity with standard integer types.
    - _Floating-Point Numbers:_ These present significant challenges. [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) allows multiple binary representations (e.g., half, single, double precision), and the same value might be representable in several. Special values like NaN (Not a Number) can have different binary payloads, and positive zero (+0) and negative zero (−0) have distinct representations. Deterministic schemes must specify canonical forms, such as always using the shortest valid representation and defining a single canonical NaN.

- **String Encoding & Unicode:** While UTF-8 is the dominant encoding today, subtleties remain. The most significant is Unicode normalization. A single character with an accent (like 'é') can often be represented either as a single precomposed character (U+00E9) or as a base character ('e', U+0065) followed by a combining accent mark (U+0301). These result in different byte sequences but represent the same visual character. Some canonicalization schemes require normalizing strings to a specific form (like NFC or NFD) before encoding, while others, like JCS and CBOR CDE, explicitly _avoid_ this step, considering it an application-level concern due to complexity and potential information loss.

- **Indefinite Lengths:** Formats like CBOR allow encoding arrays, maps, and strings without specifying their length upfront, using a special "break" marker to signal the end. This "indefinite-length" encoding is useful for streaming but introduces non-determinism, as the same data could be encoded with either a definite or indefinite length. Deterministic schemes like CBOR CDE typically disallow indefinite-length items.

- **Default Values / Optional Fields:** In formats like Protobuf, if a field is set to its default value (e.g., an integer field to 0), it might be omitted entirely during serialization, or it might be explicitly included. Since the deserialized result is the same in either case (the field has the default value), this creates representational ambiguity. Deterministic schemes often require omitting default values, similar to how [ASN. DER](https://en.wikipedia.org/wiki/X.690#DER_encoding) forbids encoding default values.

- **Extensibility Issues (Tags/Unknown Fields):** How a format handles data not explicitly defined in its schema can impact determinism. Protobuf preserves "unknown fields" encountered during parsing, which aids forward/backward compatibility but significantly hinders canonicalization because the type (and thus canonical representation) of these fields isn't known. CBOR uses tags for extensibility; while the content _within_ a tag might be canonicalized according to standard rules, ensuring that application-level data is consistently mapped _to_ specific tags and representations might require additional application-specific rules (sometimes called Application-Level Deterministic Representation or ALDR).

It becomes clear that achieving deterministic encoding involves navigating a spectrum of choices. At one end lies basic, potentially non-deterministic serialization. Moving along the spectrum, we encounter implementation-specific determinism (where a single library might be deterministic but not interoperable), recommended practices like CBOR's Preferred or Basic Serialization, and finally, fully specified canonical forms like ASN.1 DER, JCS, BCS, or CBOR CDE, which aim for a single, universally verifiable representation. The choice of where to be on this spectrum depends heavily on the application's requirements for consistency, interoperability, and security.

## 1.3 The Motivation: Why Determinism is Crucial

The quest for deterministic encoding is driven by the significant problems that arise from its absence. When the same logical data can manifest as different byte sequences unpredictably, it introduces a subtle but pervasive form of non-determinism into computing systems, leading to a range of issues that can be difficult for engineers to anticipate and resolve.

One major consequence is the emergence of **hard-to-diagnose bugs**. Systems relying on byte-wise comparisons or hashing of serialized data may fail intermittently or produce inconsistent results depending on factors like which library version is used, the internal state of the serializer (e.g., hash table iteration order affecting map key output), or even timing variations. Debugging such issues is challenging because the root cause lies not in the application logic itself, but in the seemingly innocuous step of data serialization. Failures might appear non-reproducible until the underlying serialization variability is understood and controlled.

Furthermore, non-deterministic serialization can **undermine security guarantees**. Digital signatures, for instance, rely on the verifier being able to compute the exact same hash of the message data as the signer. If the data is re-serialized between signing and verification, and the serialization is non-deterministic, the hashes will mismatch, causing valid signatures to fail verification. This not only breaks functionality but could potentially be exploited in certain scenarios. Similarly, consensus protocols in distributed systems depend on nodes agreeing on the state based on identical data representations; non-determinism breaks this agreement.

**Inefficiency** is another consequence. Caching mechanisms often use hashes of data as keys. If logically identical data produces different serialized forms (and thus different hashes), caches will suffer unnecessary misses, leading to redundant computation or data transfer. Content-addressable storage systems lose their deduplication benefits if identical content doesn't serialize identically.

Finally, non-determinism severely **hinders interoperability**. If different systems, or even different versions of the same software, serialize the same data differently, they may be unable to reliably communicate or agree on shared state. This is particularly problematic in heterogeneous environments or long-lived systems where components evolve independently. Protocol Buffers' documentation explicitly warns that its default "deterministic" mode is not canonical across languages or versions precisely for these reasons.

These diverse problems highlight a fundamental point: non-determinism in serialization erodes the foundation of trust in computational processes. Digital systems rely on predictable, repeatable behavior to function correctly and securely. When the basic representation of data—its byte sequence—becomes unpredictable for the same logical content, the operations built upon that representation (comparison, hashing, verification, agreement) become inherently unreliable. This variability undermines the integrity and dependability required for critical applications, from secure communication and financial transactions to distributed databases and verifiable records. Achieving deterministic, canonical encoding is therefore essential for building systems where computational results can be consistently verified and trusted. The need for deterministic processes is not unique to serialization; it's a recurring theme in diverse fields like coding theory, machine learning, and state machine design, reflecting a general need for predictable and reliable computation.

## 1.4 Key Use Cases Demanding Determinism

The need for deterministic encoding is not theoretical; it is driven by the practical requirements of numerous critical computing applications. Several key use cases fundamentally depend on the ability to produce a consistent, predictable byte representation for data.

### 1.4.1 Distributed Consensus

Distributed systems, ranging from replicated databases to modern blockchain networks, rely on consensus algorithms (such as Paxos, Raft, or variants of Byzantine Fault Tolerance (BFT)) to ensure that multiple independent nodes agree on a single, consistent state or order of operations. This agreement process frequently involves nodes proposing, validating, and replicating data structures like transaction logs, state updates, or proposed blocks.

A core requirement for these algorithms is that all non-faulty nodes must reach the same decision based on the same information. Often, this involves nodes independently processing received data, serializing it (or parts of it), and then hashing the result to compare with hashes received from other nodes or to include in subsequent proposals. If the serialization process is non-deterministic, two nodes processing the _exact same logical transaction or block data_ could generate different byte sequences. These different sequences would produce different cryptographic hashes, leading the nodes to disagree, even though they started with identical information. This disagreement prevents the system from reaching consensus, potentially halting progress or leading to inconsistent states across nodes.

Blockchains are a prominent example where this is critical. In a decentralized network without a central authority, nodes must independently verify transactions and agree on the contents of new blocks to add to the chain. This verification relies heavily on cryptographic hashing and consistent data representation. Deterministic serialization ensures that all nodes compute the same hashes for the same transactions and blocks, enabling the consensus mechanism (whether Proof-of-Work, Proof-of-Stake, or BFT-based) to function correctly and maintain the integrity of the shared ledger. Formats like Binary Canonical Serialization (BCS) were explicitly designed with this use case in mind, providing guaranteed byte consistency for consensus in blockchain environments.

In essence, for decentralized systems that establish trust algorithmically through consensus protocols, deterministic encoding is not merely a technical optimization but a foundational requirement. It ensures that all participants operate on verifiably identical representations of shared data, making algorithmic agreement possible and enabling trust in the absence of a central coordinator. Without it, the entire model of decentralized consensus breaks down.

### 1.4.2 Verifiable Data and Digital Signatures

Digital signatures are a cornerstone of modern digital security, providing three key properties:

- **Authenticity:** Verifying the identity of the signer.
- **Integrity:** Ensuring the data has not been altered since it was signed.
- **Non-repudiation:** Preventing the signer from later denying that they signed the data.

The process typically involves creating a cryptographic hash (a fixed-size digest) of the data to be signed, and then encrypting this hash using the signer's private key. To verify the signature, a recipient recalculates the hash of the received data using the same hash algorithm, decrypts the received signature using the signer's public key, and compares the recalculated hash with the decrypted hash. If they match, the signature is valid.

This entire process hinges on one critical assumption: both the signer and the verifier must be able to produce the _exact same hash_ from the _same logical data_. Since cryptographic hashes are extremely sensitive to input changes (a single bit flip drastically changes the output), the byte sequence fed into the hash function must be identical for both parties.

If the data is serialized non-deterministically, the signer might serialize the data one way, calculate a hash, and sign it. The verifier might receive the same logical data, but upon re-serializing it (perhaps using a different library or version), obtain a different byte sequence. This different byte sequence will produce a different hash, causing the signature verification to fail, even though the data's integrity was never compromised and the signature itself is cryptographically sound. This necessitates a deterministic, canonical representation of the data _before_ hashing and signing.

This requirement is crucial for applications like Verifiable Credentials (VCs), where data integrity proofs (often digital signatures) are used to ensure the authenticity and tamper-evidence of claims. Standards like the [W3C Data Integrity specification](https://www.w3.org/TR/vc-data-integrity/) explicitly involve transforming data into a canonical form before hashing and signing/proving.

An important advantage of using canonicalization in this context is that it decouples the format used for signing from the format used for transmission or storage. Data can be signed based on its canonical form, but then transmitted or displayed in a more convenient, possibly non-canonical format (e.g., pretty-printed JSON for readability). The verifier simply needs to re-canonicalize the received data according to the agreed-upon rules before performing the verification step. This avoids forcing systems to use potentially inefficient or human-unfriendly formats solely for the purpose of signing, offering flexibility without sacrificing security.

### 1.4.3 Content-Addressable Systems and Caching

**Content-Addressable Storage (CAS)** is a storage paradigm where data is identified and retrieved based on a cryptographic hash of its content, rather than a user-assigned name or location (like a file path). The hash acts as the unique address for the data. This approach inherently relies on deterministic encoding: the same content must always produce the same hash to be reliably stored and retrieved.

CAS offers several significant advantages:

- **Automatic Deduplication:** If the same piece of content is stored multiple times, it will always generate the same hash. CAS systems recognize this and store the actual data only once, simply adding references to the existing content. This can lead to substantial storage savings, especially in backup systems or large datasets with redundant information.
- **Data Integrity Verification:** The content hash serves as a built-in checksum. When data is retrieved, its hash can be recalculated and compared to the requested address (hash). A mismatch immediately indicates data corruption.
- **Suitability for Distributed Systems:** Content addressing works well in distributed or decentralized environments (like [IPFS](https://ipfs.tech/) or [Git](https://git-scm.com/)) because data can be located and retrieved based solely on its hash, without needing a central directory or knowledge of specific server locations.

Deterministic encoding underpins the reliability of CAS. If serialization were non-deterministic, identical logical content could produce different hashes, defeating deduplication and potentially causing data retrieval issues. Furthermore, trustworthy deduplication relies on the guarantee that only truly identical data maps to the same hash. While cryptographic hash collisions are extremely rare with strong functions, non-deterministic serialization could theoretically create attack vectors if an adversary could manipulate the serialization process to force a hash collision between different logical data, potentially tricking a system into retrieving incorrect information. Deterministic encoding ensures that the hash reliably represents the logical content, making deduplication both efficient and secure.

Similarly, **caching mechanisms** benefit greatly from deterministic encoding. Hashes derived from canonical representations of data serve as excellent cache keys. When a system needs to check if a piece of data (e.g., a database query result, a complex object, a web resource bundle) is already in the cache, it can compute the canonical hash of the data and look it up. If the serialization were non-deterministic, logically identical data might produce different hashes upon subsequent requests, leading to cache misses and forcing redundant computations or data fetches. [Content-addressable web bundles](https://adlrocha.substack.com/p/adlrocha-webbundles-are-built-for), for example, leverage this principle to improve browser cache efficiency by ensuring that a bundle's content hash only changes if the content itself changes. Deterministic behavior is also a sought-after property in lower-level caching systems within hardware and operating systems to ensure predictable performance.

### 1.4.4 Other Applications

Beyond these major areas, deterministic encoding provides benefits in several other contexts:

- **Secure Comparison and Fingerprinting:** Comparing large datasets or complex objects for equality can be done efficiently and securely by comparing the hashes of their canonical representations. This avoids transmitting the full data and ensures that only truly identical data matches. This is useful for verifying configuration consistency, detecting changes in stored records, or fingerprinting data for various tracking purposes.
- **Testing and Diagnostics:** In automated testing, ensuring that a given input always produces the exact same byte output simplifies verification, allowing for simple byte-wise comparisons of expected versus actual results. For diagnostics, presenting logged data or system states in a canonical form minimizes inconsequential differences (like map key order), making it easier for humans or tools to spot meaningful changes. It can also help in reproducing bugs that might otherwise seem non-deterministic due to variations introduced by serialization.
- **Object Hashing:** Creating consistent, cross-language hash values for complex, nested data structures (often represented as combinations of lists, maps, and primitive types in memory) requires a canonical representation strategy. This is essential for using such objects reliably in hash tables or other contexts requiring stable identifiers derived from the object's state. Naive approaches like hashing the default string representation often fail due to non-determinism.

## 1.5 The Challenges of Achieving Determinism

While the need for deterministic encoding is clear, achieving it presents several non-trivial technical challenges. These stem from the inherent ambiguities in data representation and the need to impose strict, unambiguous rules across diverse platforms and implementations. Overcoming the sources of non-determinism identified earlier requires careful algorithmic design and often involves trade-offs.

- **Map Key Sorting:** Defining a consistent order for map keys requires specifying a stable sorting algorithm that works identically everywhere. Lexicographical sorting is a common choice. However, the details matter: should the sort operate on the raw key strings (e.g., based on UTF-16 code units, as in JCS) or on the encoded byte representation of the keys (as in CBOR CDE)? Each choice has implications for implementation complexity and performance. Furthermore, sorting adds computational overhead compared to simply iterating through a map's elements in whatever order the underlying implementation provides.

- **Floating-Point Representation:** The complexities of IEEE 754 floating-point arithmetic make canonicalization difficult. Rules must precisely define how to handle different precisions (half, single, double), ensuring the shortest valid representation is chosen. Canonical forms must be defined for special values like `NaN` (potentially collapsing different `NaN` payloads into one) and distinguishing `+0` from `−0`. An additional complication is that floating-point calculations themselves can sometimes yield slightly different results across different hardware platforms or compiler optimizations, meaning the values _input_ to the serializer might differ even before encoding rules are applied.

- **Number Representation Ambiguity:** A value like '42' could potentially be represented as a standard integer, a floating-point number, or even a bignum in some formats. A canonical scheme must provide unambiguous rules for choosing the representation, such as always preferring the simplest integer type if the value fits.

- **Unicode Normalization:** Deciding how to handle different Unicode representations of the same visual character is a significant challenge. Enforcing a specific Normalization Form (like NFC or NFD) ensures that visually identical strings have the same canonical byte sequence, but it adds a potentially costly processing step and might not be desirable in all applications (e.g., if preserving the exact original byte sequence is important). Schemes like JCS and CBOR CDE deliberately omit mandatory Unicode normalization, pushing the responsibility to the application layer if needed. This simplifies the canonicalization protocol but means that `{"café": 1}` and `{"cafe\u0301": 1}` might have different canonical forms despite looking identical.

- **Handling Extensibility:** Integrating extensibility mechanisms (like Protobuf's unknown fields or CBOR's tags) with canonicalization is difficult. Preserving unknown fields, crucial for Protobuf's compatibility story, fundamentally conflicts with canonicalization because their type and structure aren't known. For CBOR tags, while the tag's content can often be canonicalized using standard rules, ensuring the consistent use and representation of the tags themselves often requires application-level agreements (ALDR) beyond the scope of the core canonical encoding specification. A truly universal canonical format might need to restrict or disallow unknown data or require extensions to define their own canonicalization rules.

- **Performance Overhead:** Implementing the rules required for canonicalization—sorting keys, normalizing numbers, checking for shortest forms, potentially normalizing Unicode—inevitably adds computational cost compared to simpler serialization methods that don't enforce these constraints. This overhead might be negligible in many applications but can be significant in high-throughput or resource-constrained environments.


This inherent trade-off between the robustness offered by canonicalization and the potential performance impact is a key consideration. Systems must carefully evaluate their specific needs. The desire for performance often leads developers to use simpler, potentially non-deterministic serialization methods by default. This explains why canonical encoding isn't universally applied and why formats like CBOR offer different levels of determinism (Preferred, Basic, CDE), allowing applications to choose the appropriate balance between strictness and speed.

## 1.6 Surveying the Landscape: Previous Efforts

Over the years, various efforts have been made to address the need for deterministic or canonical representations, particularly for common data formats used in distributed systems and security protocols. Examining these provides valuable context and highlights recurring patterns and challenges.

### 1.6.1 JSON's Canonicalization Conundrum

JSON (JavaScript Object Notation), despite its ubiquity, lacks a built-in canonical representation defined in its base specification ([RFC 8259](https://datatracker.ietf.org/doc/html/rfc8259)). This omission means that naive serialization of JSON objects can easily lead to non-deterministic output due to varying property order and potential whitespace differences.

Several approaches have emerged to fill this gap:

- **RFC 8785: JSON Canonicalization Scheme (JCS):** This is arguably the most prominent standard for JSON canonicalization. JCS achieves determinism by defining a strict set of rules:

    - **Data Subset:** Input JSON must conform to the I-JSON profile ([RFC 7493](https://datatracker.ietf.org/doc/html/rfc7493)), which disallows duplicate object keys and imposes limits on number precision.
    - **Primitive Serialization:** Relies on the well-defined serialization of primitives (strings, numbers, booleans, null) specified by ECMAScript. Whitespace between tokens is forbidden.
    - **String Handling:** Specifies precise escaping rules for control characters and special characters like backslash and double-quote. Notably, it does _not_ mandate Unicode normalization.
    - **Number Handling:** Numbers are serialized according to ECMAScript rules, effectively using IEEE 754 double-precision representation.
    - **Object Key Sorting:** Object properties MUST be sorted recursively based on the lexicographical order of their keys, comparing the keys as sequences of UTF-16 code units.
    - **Array Element Order:** The order of elements within JSON arrays is preserved.
    - **Encoding:** The final output must be UTF-8 encoded.

    JCS is published as an Informational RFC, meaning it's not an IETF standard but represents a community consensus. It has seen adoption in specific contexts, such as for JSON Web Key (JWK) Thumbprints ([RFC 7638](https://www.rfc-editor.org/rfc/rfc7638.html)) and systems like Keybase, and libraries exist in multiple languages. However, it is not universally adopted across the JSON ecosystem, leading to a degree of fragmentation where applications might implement their own ad-hoc canonicalization or use different schemes.

- **ObjectHash:** This represents a different philosophy. Instead of producing a canonical _text_ representation, [ObjectHash](https://github.com/benlaurie/objecthash) computes a cryptographic hash directly from the semantic structure of a JSON-like object (lists, dictionaries, primitives). It defines specific hashing procedures for each type, including sorting dictionary keys before hashing. A key feature is its support for redaction: parts of a structure can be replaced by the hash of the redacted part, allowing verification of the overall structure even with hidden data. This approach avoids intermediate text serialization altogether.

- **Other Ad-hoc Methods:** Many systems implement simpler, non-standardized canonicalization, often just involving sorting object keys alphabetically before using a standard JSON serializer. While better than no canonicalization, these methods lack the precise rules for primitive serialization found in JCS and may not be interoperable.


The situation for JSON highlights the difficulty of retrofitting canonicalization onto a widely adopted, flexible format without a single, mandated standard.

**Comparison of Selected JSON Canonicalization Approaches**

|Feature|RFC 8785 JCS|ObjectHash|Ad-hoc Sorting + JSON.stringify|
|---|---|---|---|
|**Approach**|Canonical Text Serialization|Direct Cryptographic Hashing of Structure|Text Serialization after Key Sort|
|**Output**|UTF-8 JSON Text|Cryptographic Hash (e.g., SHA-256)|JSON Text (often UTF-8)|
|**Basis**|RFC 8785 (Informational)|Custom Specification|Application-specific|
|**Object/Map Key Ordering**|Mandatory Lexicographical Sort (UTF-16 units)|Mandatory Lexicographical Sort (before hashing)|Typically Lexicographical Sort (implementation varies)|
|**Number Handling**|ECMAScript standard (IEEE 754 double)|Specific hashing rule for numbers|Depends on underlying JSON serializer|
|**String Handling**|ECMAScript standard; No Unicode Normalization|Specific hashing rule; No Unicode Normalization|Depends on underlying JSON serializer|
|**Extensibility/Unknowns**|Constrained by I-JSON; No explicit unknown handling|Handles basic JSON types; Redaction mechanism|Depends on underlying JSON serializer|
|**Key Features**|Interoperable text form, Cryptographic use|Redactability, Avoids text intermediate|Simplicity (potentially fragile)|
|**Adoption Notes**|Used in JWK Thumbprint, Keybase; Libraries exist|Used in specific projects (e.g., Certificate Transparency logs)|Common but non-standardized|

### 1.6.2 ASN.1 Distinguished Encoding Rules (DER)

Abstract Syntax Notation One ([ASN.1](https://en.wikipedia.org/wiki/ASN.1)) is a mature standard from the ITU-T for defining data structures, widely used in telecommunications and security protocols. Associated with ASN.1 are several encoding rule sets that specify how to serialize data structures into bytes. The most relevant for canonicalization is the **Distinguished Encoding Rules (DER)**, specified in ITU-T X.690.

DER is a specialized subset of the more flexible Basic Encoding Rules (BER). While BER allows multiple ways to encode the same value (e.g., different length specifications, constructed vs. primitive string forms), DER restricts these choices to ensure that any given ASN.1 value has exactly one valid DER encoding. This canonical property is achieved primarily through restriction: DER mandates specific choices where BER offers flexibility:

- **Length Encoding:** Length fields must use the definite form and the minimum possible number of octets. Indefinite lengths (allowed in BER) are prohibited.
- **String Types:** Primitive encoding must be used for string types like OCTET STRING and BIT STRING (BER allows constructed forms). Unused bits in the final octet of a BIT STRING must be zero.
- **Boolean Values:** FALSE must be encoded as a single byte `0x00`, and TRUE as a single byte `0xFF`.
- **Set Ordering:** Elements within a SET OF construct must be sorted according to their tag value and encoded bytes.
- **Default Values:** Fields with default values defined in the ASN.1 schema must NOT be encoded if they hold the default value.

The primary application of DER is in Public Key Infrastructure (PKI), particularly for encoding **X.509 digital certificates** and Certificate Revocation Lists (CRLs). The unambiguous nature of DER is critical for ensuring that certificates can be parsed and validated consistently across different systems and that digital signatures covering certificate contents are reliable.

DER's success lies in its long-standing use and effectiveness within its specific domain (PKI). It demonstrates that canonicalization can be achieved and maintained over decades. However, ASN.1 and DER are often perceived as complex and potentially verbose compared to more modern formats like JSON or CBOR, which has limited their adoption in web-centric APIs and applications.

### 1.6.3 Hashing and Signing Strategies

The interaction between data serialization and cryptographic operations like hashing and digital signing is a critical area where determinism is paramount. The overwhelmingly standard practice is to **canonicalize the data first, then apply the cryptographic operation (hash or signature) to the resulting canonical byte stream**.

Signing non-canonical data introduces significant risks. A signature created over one specific byte representation might be valid only for that exact sequence. If the recipient re-serializes the data differently (due to non-deterministic rules), the signature verification will fail, even if the logical data is unchanged. This can lead to false integrity failures or, in more complex scenarios, potentially allow an attacker to craft a different serialization of the same logical data that bypasses certain checks while still matching the original signature under lenient verification rules. Canonicalization before signing ensures that the signature is bound to the _semantic content_ rather than a specific, potentially fragile, byte layout.

It is important to distinguish the canonicalization of the _message_ input from the determinism of the _signature algorithm_ itself. Some signature algorithms, like DSA and ECDSA, traditionally required a random number (nonce) for each signature. Flaws in the random number generation process have historically led to catastrophic private key compromises. To mitigate this, **deterministic signature generation** schemes have been developed, such as RFC 6979 for DSA/ECDSA and the inherent design of algorithms like Ed25519. These schemes derive the necessary nonce deterministically from the private key and the message hash, eliminating the need for external randomness during signing.

Therefore, achieving robust and reliable digital signatures often involves ensuring determinism at two distinct layers:

1. **Deterministic Message Representation:** Using canonicalization to ensure the input to the hash function is consistent.
2. **Deterministic Signature Computation:** Using algorithms that derive internal randomness (like nonces) deterministically to avoid reliance on potentially flawed external random sources. Both layers address different potential failure points and contribute to the overall security and reliability of the signing process.

### 1.6.4 Other Binary Formats (Brief Mention)

While JSON and ASN.1/DER represent major text and schema-driven binary approaches, other binary formats also grapple with determinism:

- **Protocol Buffers (Protobuf):** As mentioned earlier, Protobuf offers a "deterministic serialization" mode. However, the documentation clearly states this is _not_ canonical. It guarantees byte consistency only for a specific binary build and schema version, but not across different language implementations, library versions, or even schema evolution (due to the handling of unknown fields). Its design prioritizes compatibility and efficiency over strict canonicalization. Specific deterministic schemes have been layered on top for specific use cases, like Cosmos SDK's ADR-027, which adds rules for field ordering, varint encoding, and default value handling.

- **Binary Canonical Serialization (BCS):** In contrast to Protobuf, [BCS](https://move-book.com/programmability/bcs.html) was explicitly designed from the ground up with canonicalization as a primary goal. Originating in the Diem blockchain project (defunct) and now used widely in the Move language ecosystem (Sui, Aptos), BCS aims for simplicity, efficiency, and a guaranteed one-to-one mapping between in-memory values and byte representations. It defines strict rules for encoding primitives (little-endian integers, ULEB128 for lengths), sequences, maps (implicitly requiring ordered keys for canonicalization, though the base spec focuses more on structs), and structs (fields serialized in definition order). Its primary motivation is to support cryptographic hashing and consensus mechanisms in blockchains.


## 1.7 Lessons Learned: Successes, Shortcomings, and the Path Forward

The survey of deterministic and canonical encoding efforts reveals valuable lessons about what has worked, what challenges persist, and what properties are desirable in future solutions.

**Successes:**

- **Domain-Specific Stability:** ASN.1 DER demonstrates that a strict canonical encoding standard can achieve long-term stability and interoperability within a well-defined domain like PKI, serving as the foundation for X.509 certificates for decades.
- **Addressing Common Formats:** Efforts like JCS (RFC 8785) provide a viable, albeit not universally adopted, solution for canonicalizing JSON, leveraging existing widespread technologies like ECMAScript for primitive serialization.
- **Purpose-Built Solutions:** Formats like BCS show that when canonicalization is a primary design goal, especially for demanding use cases like blockchain consensus, efficient and effective binary formats can be created.
- **Enabling Critical Patterns:** Canonicalization is demonstrably essential for enabling robust digital signatures, reliable distributed consensus, and efficient content-addressable storage and caching.

**Shortcomings and Persistent Challenges:**

- **Fragmentation:** For popular, flexible formats like JSON, the lack of a single, universally mandated canonicalization standard leads to fragmentation, with multiple competing schemes or ad-hoc solutions.
- **Design Conflicts:** Some formats, like Protocol Buffers, have core design features (e.g., handling of unknown fields for compatibility) that inherently conflict with the requirements for true canonicalization across different contexts.
- **Complexity and Overhead:** Achieving canonicalization often introduces complexity in implementation and runtime overhead due to steps like sorting or normalization, creating a trade-off against performance. This can make canonical forms less appealing for performance-critical applications if the benefits are not strictly required. ASN.1/DER, while successful, is often perceived as overly complex for simpler web-based use cases.
- **Handling Nuances:** Accurately and consistently handling the subtleties of floating-point arithmetic and Unicode across all platforms remains a persistent challenge requiring explicit and careful rule definition.
- **Extensibility:** Integrating extensibility mechanisms (like tags or user-defined types) into a canonical framework without requiring constant updates to the core specification or relying heavily on application-level agreements remains difficult.

A recurring theme emerges from these observations: the inherent tension between designing a data format for maximum flexibility and extensibility, and achieving strict, simple canonicalization. Features enhancing flexibility—multiple number encodings, optional fields, variable map ordering, mechanisms for unknown data—often introduce the very ambiguities that canonicalization seeks to eliminate. Consequently, canonical formats frequently achieve their goal by _restricting_ the flexibility of the underlying data model or base encoding rules (DER restricts BER, CDE restricts CBOR, JCS restricts JSON via I-JSON). Designing a format that balances flexibility with ease of canonicalization requires careful consideration from the outset.

The Path Forward:

The increasing prevalence of distributed systems, the demand for verifiable data (like VCs), and the constant need for robust security mechanisms ensure that the need for reliable deterministic and canonical encoding will only grow. An ideal solution, building on the lessons learned, should strive for:

- **Unambiguity:** Clear, precise rules that leave no room for interpretation and lead to a single, verifiable canonical form.
- **Efficiency:** Minimize computational overhead compared to non-canonical serialization, making it practical for a wider range of applications.
- **Simplicity:** Easy to understand and implement correctly, reducing the likelihood of errors (a key goal of CBOR itself).
- **Robustness:** Handle common data types, including integers, floating-point numbers, and strings (with clear rules regarding Unicode), effectively.
- **Well-Defined Extensibility:** Provide a clear path for extending the format without breaking canonical properties or requiring constant core specification updates.

These desirable properties set the stage for exploring more advanced solutions designed to meet these needs within the context of modern data formats. The subsequent chapters will delve into how dCBOR aims to provide such a solution within the CBOR ecosystem.
