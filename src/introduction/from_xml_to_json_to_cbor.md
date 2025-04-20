# From XML to JSON to CBOR

## A *Lingua Franca* for Data?

In modern computing, data exchange is foundational to everything from web browsing to microservices and IoT devices. The ability for different systems to represent, share, and interpret structured information drives our digital world. Yet no single perfect format has emerged to meet all needs. Instead, we've seen an evolution of data interchange formats, each addressing the specific challenges and technical requirements of its time.

This narrative traces three pivotal data formats: Extensible Markup Language ([XML](https://www.w3.org/TR/xml/)), JavaScript Object Notation ([JSON](https://www.json.org/)), and Concise Binary Object Representation ([CBOR](https://cbor.io/)). We explore their origins and motivations, examine their core design principles and inherent trade-offs, and follow their adoption trajectories within the evolving digital landscape. The journey begins with XML's focus on robust document structure, shifts to JSON's web-centric simplicity and performance, and advances to CBOR's binary efficiency for constrained devices. Understanding this evolution reveals not just technical specifications, but the underlying pressures driving innovation in data interchange formats.

## The Age of Structure: XML's Rise from Publishing Roots

Modern data interchange formats trace back not to the web, but to challenges in electronic publishing decades earlier. SGML provided the complex foundation that XML would later refine and adapt for the internet age.

### The SGML Inheritance: Laying the Foundation

In the 1960s-70s, IBM researchers Charles Goldfarb, Ed Mosher, and Ray Lorie created Generalized Markup Language (GML) to overcome proprietary typesetting limitations. Their approach prioritized content structure over presentation. GML later evolved into Standard Generalized Markup Language (SGML), formalized as ISO 8879 in 1986.

SGML innovated through its meta-language approach, providing rules for creating custom markup languages. It allowed developers to define specific vocabularies (tag sets) and grammars (Document Type Definitions or DTDs) for different document types, creating machine-readable documents with exceptional longevity independent of processing technologies.

SGML gained traction in sectors managing complex documentation: government, military (CALS DTD), aerospace, legal publishing, and heavy industry. However, its 150+ page specification with numerous special cases complicated parser implementation, limiting broader adoption.

The web's emergence proved pivotal for markup languages. Tim Berners-Lee selected SGML as HTML's foundation due to its text-based, flexible, non-proprietary nature. Dan Connolly created the first HTML DTD in 1992. While HTML became ubiquitous, it drifted toward presentation over structure, with proliferating browser-specific extensions. SGML remained too complex for widespread web use, creating demand for a format that could bring SGML's structural capabilities to the internet in a more accessible form.

### W3C and the Birth of XML: Taming SGML for the Web

By the mid-1990s, the web needed more structured data exchange beyond HTML's presentational focus. In 1996, the W3C established an XML Working Group, chaired by Jon Bosak of Sun Microsystems, to create a simplified SGML subset suitable for internet use while maintaining extensibility and structure.

The W3C XML Working Group developed XML with clear design goals, formalized in the XML 1 Specification (W3C Recommendation, February 1998):

1. **Internet Usability**: Straightforward use over the internet
2. **Broad Applicability**: Support for diverse applications beyond browsers
3. **SGML Compatibility**: XML documents should be conforming SGML documents
4. **Ease of Processing**: Simple program development for XML processing
5. **Minimal Optional Features**: Few or no optional features
6. **Human Readability**: Legible and clear documents
7. **Rapid Design**: Quick design process
8. **Formal and Concise Design**: Formal specification amenable to standard parsing
9. **Ease of Creation**: Simple document creation with basic tools
10. **Terseness is Minimally Important**: Conciseness was not prioritized over clarity

SGML compatibility was strategically crucial. By defining XML as a valid SGML subset, existing SGML parsers and tools could immediately process XML documents when the standard released in 1998. This lowered adoption barriers for organizations already using SGML and provided an instant software ecosystem. The constraint also helped the working group achieve rapid development by limiting design choices, demonstrating an effective strategy for launching the new standard.

### Designing XML: Tags, Attributes, Namespaces, and Schemas

XML's structure uses nested elements marked by tags. An element consists of a start tag (`<customer>`), an end tag (`</customer>`), and content between them, which can be text or other nested elements. Start tags can contain attributes for metadata (`<address type="billing">`). Empty elements use syntax like `<br/>` or `<br></br>`. This hierarchical structure makes data organization explicit and human-readable.

As XML usage expanded, combining elements from different vocabularies created naming conflicts. The "Namespaces in XML" Recommendation (January 1999) addressed this by qualifying elements with unique IRIs, typically URIs. This uses the `xmlns` attribute, often with a prefix (`xmlns:addr="http://www.example.com/addresses"`), creating uniquely identified elements (`<addr:street>`). Default namespaces can be declared (`xmlns="URI"`) for unprefixed elements, but don't apply to attributes. Though URIs ensure uniqueness, they needn't point to actual online resources.

XML documents are validated using schema languages. XML initially used Document Type Definitions (DTDs) from SGML, which define allowed elements, attributes, and nesting rules. To overcome DTD limitations (non-XML syntax, poor type support), the W3C developed XML Schema Definition ([XSD](https://www.w3.org/TR/xmlschema11-1/)), standardized in 2001. XSD offers powerful structure definition, rich data typing, and rules for cardinality and uniqueness. XSD schemas are themselves written in XML.

XML's structure enabled supporting technologies: XPath for node selection, XSL Transformations ([XSLT](https://www.w3.org/TR/xslt20/)) for document transformation, and APIs like Document Object Model (DOM) for in-memory representation or Simple API for XML (SAX) for event-based streaming.

While XML effectively modeled complex data structures with extensibility and validation, its power introduced complexity. Creating robust XSD schemas was challenging, leading some to prefer simpler alternatives like RELAX NG or Schematron. Namespaces solved naming collisions but complicated both document authoring and parser development. XML's flexibility allowed multiple valid representations of the same data, potentially hindering interoperability without strict conventions. This inherent complexity, combined with verbosity, eventually drove demand for simpler formats, especially where ease of use and performance outweighed validation and expressiveness. The tension between richness and simplicity significantly influenced subsequent data format evolution.

### XML's Reign and Ripples: Adoption and Impact

Following its 1998 standardization, XML quickly became dominant across computing domains throughout the early 2000s, offering a standard, platform-independent approach for structured data exchange.

XML formed the foundation of **Web Services** through SOAP (Simple Object Access Protocol), an XML-based messaging framework operating over HTTP. Supporting technologies like WSDL (Web Services Description Language) and UDDI (Universal Description, Discovery and Integration) completed the "WS-*" stack for enterprise integration.

**Configuration Files** widely adopted XML due to its structure and readability. Examples include Java's Log4j, Microsoft.NET configurations (`web.config`, `app.config`), Apache Ant build scripts, and numerous system parameters.

In **Document Formats and Publishing**, XML fulfilled its original promise by powering XHTML, RSS and Atom feeds, KML geographic data, and specialized formats like DocBook. Its content-presentation separation proved valuable for multi-channel publishing and content management.

As a general-purpose **Data Interchange** format, XML facilitated cross-system communication while avoiding vendor lock-in and supporting long-term data preservation.

This widespread adoption fostered a rich ecosystem of XML parsers, editors, validation tools, transformation engines (XSLT), data binding utilities, and dedicated conferences, building a strong technical community.

### The Seeds of Change: XML's Verbosity Challenge

Despite its success, XML carried the seeds of its own partial decline. A key design principle—"Terseness in XML markup is of minimal importance"—prioritized clarity over compactness, requiring explicit start and end tags for every element.

While enhancing readability, this structure created inherent verbosity. Simple data structures required significantly more characters in XML than in more compact formats. For example, `{"name": "Alice"}` in JSON versus `<name>Alice</name>` in XML added substantial overhead, especially for large datasets with many small elements.

This verbosity became problematic as the web evolved. The rise of AJAX in the mid-2000s emphasized frequent, small data exchanges between browsers and servers for dynamic interfaces. In this context, minimizing bandwidth usage and parsing time became critical. XML's larger payloads and complex parsing requirements created performance bottlenecks.

The XML community recognized these efficiency concerns, leading to initiatives like the W3C's Efficient XML Interchange (EXI) Working Group, which developed a standardized binary XML format. While EXI offered significant compaction, it highlighted the challenge of retrofitting efficiency onto XML's tag-oriented foundation without adding complexity.

The decision to deprioritize terseness, while distinguishing XML from SGML, had unintended consequences. As the web shifted toward dynamic applications prioritizing speed and efficiency, XML's verbose structure became a liability. This created an opportunity for a format that would optimize for precisely what XML had considered minimal: conciseness and ease of parsing within web browsers and JavaScript.

## The Quest for Simplicity: JSON's Emergence in the Web 2.0 Era

As XML's verbosity and complexity became problematic in web development, particularly with AJAX's rise, a simpler alternative emerged directly from JavaScript.

### JavaScript's Offspring: Douglas Crockford and the "Discovery" of JSON

JSON (JavaScript Object Notation) originated with Douglas Crockford, an American programmer known for his JavaScript work. In 2001, Crockford and colleagues at State Software needed a lightweight format for data exchange between Java servers and JavaScript browsers without plugins like Flash or Java applets.

Crockford realized JavaScript's object literal syntax (e.g., `{ key: value }`) could serve this purpose. Data could be sent from servers embedded in JavaScript snippets for browsers to parse, initially using the `eval()` function. Crockford describes this as a "discovery" rather than invention, noting similar techniques at Netscape as early as 1996.

The initial implementation sent HTML documents containing `<script>` tags that called JavaScript functions, passing data as object literal arguments. One refinement: all keys required double quotes to avoid conflicts with JavaScript reserved words.

After naming conflicts with JSpeech Markup Language, they settled on "JavaScript Object Notation" or JSON. In 2002, Crockford acquired [json.org](https://json.org) and published the grammar and reference parser. Developers quickly submitted parsers for various languages, demonstrating JSON's broader potential.

### Motivation: A Lightweight Alternative for a Faster Web

JSON addressed the need for a simpler, lighter data interchange format than XML. Crockford aimed for minimalism, believing "the less we have to agree on in order to inter-operate, the more likely we're going to be able to inter-operate well." He wanted a standard simple enough to fit on a business card.

When challenged that JSON was merely reinventing XML, Crockford famously replied, "The good thing about reinventing the wheel is that you can get a round one."

JSON arrived at the perfect time. AJAX techniques created demand for optimized, small data transfers between servers and browsers. Though "AJAX" meant "Asynchronous JavaScript and XML," JSON proved better for many cases. Its syntax maps directly to JavaScript objects and arrays, making client-side parsing trivial. Its lightweight nature reduced bandwidth usage and improved web application responsiveness.

Despite originating from JavaScript, JSON's success wasn't confined to browsers. Its simplicity made it remarkably easy to implement across programming languages. The core structures—objects (maps/dictionaries), arrays (lists), strings, numbers, booleans, and null—are fundamental to most modern languages. This ease of cross-language implementation drove its rapid adoption, transforming it from a JavaScript-specific solution into a de facto standard for web APIs and configuration files industry-wide. Simplicity became a powerful catalyst for language independence and widespread adoption.

### Designing JSON: Key-Value Pairs, Arrays, and Minimal Types

JSON's syntax is deliberately minimal, built on just a few structural elements from JavaScript:

- **Objects**: Unordered key-value pairs in curly braces `{}`. Keys must be double-quoted strings, followed by a colon `:`, with comma-separated pairs. Example: `{ "name": "Alice", "age": 30 }`.
- **Arrays**: Ordered value sequences in square brackets `[]`, separated by commas. Example: `[ "apple", "banana", "cherry" ]`.

Values can only be:
- **String**: Double-quoted Unicode characters
- **Number**: Numeric values (without type distinction)
- **Boolean**: `true` or `false` (lowercase)
- **Null**: `null` (lowercase)
- **Object**: Nested JSON object
- **Array**: Nested JSON array

This text-based structure is human-readable and directly maps to common programming data structures, making it developer-friendly.

JSON intentionally omits XML features like comments, namespaces, and attributes. Crockford deliberately excluded comments, noting they were often misused in formats like XML for parsing directives or metadata, potentially breaking interoperability. The recommended approach is to include commentary as regular data with conventional keys like `"_comment"`.

Native support arrived in ECMAScript 5 (2009) with `JSON.parse()` and `JSON.stringify()` methods, providing safe alternatives to `eval()` for parsing. The `stringify` method supports optional `replacer` functions for output control, and objects can implement `toJSON()` to customize serialization.

### JSON vs. XML: A Paradigm Shift

JSON and XML reflect fundamentally different design philosophies:

- **Format Type**: XML is a _markup language_ for structured documents; JSON is purely a _data interchange format_ derived from JavaScript object literals.
- **Structure**: XML uses hierarchical tags with elements, attributes, and text. JSON uses _key-value pairs_ and ordered _arrays_.
- **Verbosity**: XML's tag structure creates inherent verbosity. JSON's minimal syntax produces more compact representations, often 30-40% smaller.
- **Readability**: Both are text-based, but JSON's simpler structure is typically easier to scan and comprehend.
- **Parsing**: JSON parsing is simpler and faster, with native support in JavaScript. XML requires more complex parsers to handle tags, attributes, namespaces, and validation.
- **Features**: XML includes comments, namespaces, attributes, and robust schema languages (DTD, XSD). JSON is intentionally minimal, with extensions like [JSON Schema](https://json-schema.org/) and [JSON-LD](https://json-ld.org/) handled separately.
- **Data Types**: JSON supports basic types (string, number, boolean, null, object, array). XML lacks built-in types without schemas, but XSD enables rich typing.

This comparison reveals the shift: XML prioritized structure, extensibility, and validation for complex documents, while JSON emphasized simplicity, usability, and performance for web APIs.

### Rapid Ascent: JSON Becomes the Language of APIs

JSON's alignment with web technologies drove its widespread adoption during the "Web 2.0" and AJAX era. It quickly dominated **RESTful web APIs**, with surveys showing over 85% of APIs using JSON as their default format.

Its utility extended to **configuration files** and **data storage**, particularly in NoSQL databases like MongoDB (using [BSON](https://bsonspec.org/)) and browser storage via `localStorage`.

JSON's adoption grew organically through developer preference and the ease of creating parsers across languages, as seen in implementations at [json.org](https://json.org). Formal standardization followed with [ECMA-404](https://ecma-international.org/publications-and-standards/standards/ecma-404/) and [IETF RFC 8259](https://datatracker.ietf.org/doc/html/rfc8259).

A key factor in JSON's success is its remarkable stability. As Crockford emphasized, JSON is "finished"—it has no version number, and its core specification remains unchanged since inception. This stability contrasts with technologies requiring frequent updates, avoiding the fragmentation and compatibility issues CBOR later explicitly designed against. By providing a simple, reliable foundation, JSON allowed a rich ecosystem to flourish around it without requiring constant adaptation to core changes, proving stability to be a decisive feature for infrastructure technologies.

## The Need for Speed (and Size): Enter CBOR

While JSON offered a much-needed simplification and performance boost over XML for web APIs, its text-based nature still presented limitations in certain demanding environments. The relentless push for greater efficiency, particularly driven by the rise of the Internet of Things (IoT), paved the way for a format that combined JSON's data model with the compactness and speed of binary encoding: CBOR.

### Beyond Text: The Motivation for Binary

Text-based formats like JSON have inherent inefficiencies compared to binary representations:

- **Parsing Speed**: Text parsing requires interpreting character sequences, computationally costlier than decoding structured binary data. Binary formats map more directly to machine data types.
- **Message Size**: Numbers, booleans, and repeated keys consume more bytes as text than with optimized binary encodings. Comparisons consistently show CBOR significantly reducing data size versus JSON.
- **Binary Data Handling**: JSON lacks a native binary data type (needed for images, cryptographic keys, sensor readings). Such data requires Base64 encoding, adding complexity and increasing size by ~33%.

These limitations become critical in **constrained environments** characteristic of IoT:

- **Limited Resources**: Minimal CPU, memory, and battery power
- **Constrained Networks**: Low bandwidth, high latency connections ([LoRaWAN](https://lora-alliance.org/), [NB-IoT](https://www.gsma.com/solutions-and-impact/technologies/internet-of-things/narrow-band-internet-of-things-nb-iot/), [Bluetooth LE](https://www.bluetooth.com/))

In these scenarios, minimizing message size conserves bandwidth and energy, while reducing processing overhead extends battery life. CBOR was specifically designed to provide JSON's flexible data model in a compact, efficiently processable binary form optimized for resource-constrained environments.

### IETF Standardization: Building on the JSON Model

CBOR was developed within the IETF specifically for constrained environments, with Carsten Bormann and Paul Hoffman as key contributors.

CBOR intentionally builds upon the JSON data model, supporting equivalent types (numbers, strings, arrays, maps, booleans, and null) while adding native support for **binary byte strings** to address a key JSON limitation.

Initially standardized in RFC 7049 (2013), CBOR was updated in [RFC 8949](https://datatracker.ietf.org/doc/html/rfc8949) (2020) as Internet Standard 94 (STD 94). Importantly, RFC 8949 maintains full wire-format compatibility with its predecessor.

The standard articulates clear design goals:

- **Compact Code Size**: Implementable with minimal code footprint for memory-constrained devices
- **Reasonable Message Size**: Significantly smaller than JSON without complex compression
- **Extensibility without Version Negotiation**: Future extensions remain compatible with existing decoders
- **Schema-Free Decoding**: Self-describing data items, parsable without predefined schemas
- **Broad Applicability**: Suitable for both constrained nodes and high-volume applications
- **JSON Compatibility**: Support for JSON data types with reasonable conversion

CBOR effectively synthesizes lessons from both JSON and XML. It adopts JSON's familiar data model while optimizing for constrained environments through binary encoding and size efficiency. For extensibility, CBOR provides semantic **tags** (registered via [IANA](https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml)) that allow new data types to be incorporated without breaking backward compatibility—combining JSON's simplicity with XML's extensibility approach.

### Where CBOR Shines: Constrained Environments

CBOR has established itself primarily in **Internet of Things (IoT)** and **constrained environments** where its compact representation of complex data structures provides crucial efficiency gains.

Key IETF protocols leveraging CBOR include:

- [CoAP (Constrained Application Protocol)](https://coap.space/): A lightweight HTTP alternative for constrained networks using CBOR payloads. Mappings exist for protocols like IEC 61850 (smart grids), showing performance benefits over HTTP/XML or WS-SOAP.

- [COSE (CBOR Object Signing and Encryption)](https://datatracker.ietf.org/doc/rfc8152/): Defines cryptographic operations using CBOR, building on JOSE concepts but with binary efficiency. Fundamental to IoT security and used in FIDO WebAuthn passkey authentication.

- [ACE (Authentication and Authorization for Constrained Environments)](https://datatracker.ietf.org/doc/html/rfc9200): Security framework for IoT resource access using CBOR and COSE.

- **Device Management**: Protocols like [CORECONF](https://core-wg.github.io/comi/draft-ietf-core-comi.html) apply NETCONF/YANG concepts to constrained devices via CBOR.

- **Certificate Representation**: [C509](https://datatracker.ietf.org/doc/draft-ietf-cose-cbor-encoded-cert/) creates smaller X.509 certificates than traditional DER/ASN.1 encoding.

Beyond IETF standards, formats like [CBOR-LD](https://json-ld.github.io/cbor-ld-spec/) and [CBL](https://arxiv.org/abs/2407.04398) compress semantic web data for IoT applications.

Widespread [implementation support](https://cbor.io/impls.html) across languages (C, C++, Go, Rust, Python, Java, Swift, etc.) facilitates CBOR integration across diverse systems.

While CBOR adoption grows within constrained systems and security protocols, it remains younger than XML and less dominant than JSON in general web APIs. Its binary nature sacrifices human readability for efficiency, making it less suitable where direct inspection and manual editing are priorities.

### The Trajectory: CBOR's Place and Future

CBOR's evolution optimizes for binary efficiency while maintaining JSON's flexible data model. Its growth centers on environments where these optimizations matter most: IoT, M2M communication, and security protocols.

As billions more IoT devices deploy, demand for efficient communication will increase, strengthening CBOR's position. Its integration into security mechanisms like COSE, particularly with passwordless authentication (WebAuthn/Passkeys), drives further adoption. CBOR's semantic tags provide extensibility without breaking backward compatibility.

In Part II, we'll explore another crucial CBOR advantage: deterministic encoding. This property ensures consistent serialization, essential for cryptographic applications including signatures, hashing, secure messaging, and distributed consensus protocols.

Despite these strengths, CBOR won't likely displace JSON in web APIs and general data interchange, where human readability and JavaScript integration remain paramount advantages.

## Conclusion: An Evolving Landscape of Data Representation

The XML-JSON-CBOR evolution demonstrates technology's pattern of moving from feature-rich solutions toward specialized formats for specific use cases. SGML offered comprehensive features but complexity; XML simplified it for web documents; JSON further streamlined for web APIs; CBOR then optimized for binary efficiency in constrained environments.

The future likely holds coexistence rather than a single dominant format, with selection driven by application requirements. Specialized formats like CBOR achieve superior performance within their niches through deliberate trade-offs, such as exchanging human readability for size and processing speed.

**Comparative Overview of XML, JSON, and CBOR**

|Feature|XML|JSON|CBOR|
|---|---|---|---|
|**Originator/Body**|W3C (Jon Bosak et al.)|Douglas Crockford; later ECMA, IETF|IETF (Carsten Bormann, Paul Hoffman)|
|**Primary Goal**|Structured Docs, Web Data Exchange|Simple/Lightweight Web APIs, Data Interchange|Binary Efficiency, Compactness, Constrained Environments (IoT)|
|**Format Type**|Markup Language (Text)|Data Format (Text)|Data Format (Binary)|
|**Base Model**|SGML Subset|JavaScript Object Literal Subset|JSON Data Model Extension|
|**Structure**|Tag-based Tree (Elements, Attributes)|Key-Value Pairs (Objects) & Ordered Values (Arrays)|Key-Value Pairs (Maps) & Ordered Values (Arrays)|
|**Schema/Validation**|DTD, XSD (Built-in, Strong)|JSON Schema (Separate Spec, Optional)|CDDL (Separate Spec, Optional)|
|**Human Readability**|High (Verbose)|High (Concise)|Low (Binary)|
|**Size/Efficiency**|Verbose, Less Efficient Parsing|Lightweight, Efficient Parsing|Very Compact, Highly Efficient Parsing|
|**Extensibility**|Namespaces, Schema|Via conventions (e.g., JSON-LD), JSON Schema|Semantic Tags (IANA Registry)|
|**Native Binary Support**|No (Requires Encoding, e.g., Base64)|No (Requires Encoding, e.g., Base64)|Yes (Byte String Type)|
|**Primary Use Cases**|Documents (HTML, DocBook), SOAP, Config Files|REST APIs, Config Files, NoSQL Data|IoT Protocols (CoAP), Security (COSE), Constrained Devices|

## References

- [W3C Recommendation: Extensible Markup Language (XML) 1.0 (Fifth Edition)](https://www.w3.org/TR/xml/)
    - The foundational W3C specification defining XML.

- [IETF RFC 8259: The JavaScript Object Notation (JSON) Data Interchange Format](https://datatracker.ietf.org/doc/html/rfc8259)
    - The current IETF standard defining JSON, essential for understanding its formal specification.

- [IETF RFC 8949: Concise Binary Object Representation (CBOR)](https://datatracker.ietf.org/doc/html/rfc8949)
    - The IETF standard defining CBOR, its data model, binary encoding, and extensibility.

- [Walsh, N. "A Technical Introduction to XML"](https://nwalsh.com/docs/articles/xml/)
    - Clearly outlines the original design goals and motivations behind XML's creation.

- ["The Rise and Rise of JSON" – Two-Bit History](https://twobithistory.org/2017/09/21/the-rise-and-rise-of-json.html)
    - Provides an excellent narrative on JSON's origins, motivations, and the context of its emergence relative to XML.

- [CBOR.io (Official CBOR Website)](https://cbor.io/)
    - Authoritative overview of CBOR, its rationale, features, and links to specifications and implementations.

- [JSON.org](http://json.org/)
    - The original website by Douglas Crockford where JSON was first formally described and popularized.

- [AWS: "JSON vs XML – Difference Between Data Representations"](https://aws.amazon.com/compare/the-difference-between-json-xml/)
    - A representative comparison highlighting the practical differences and trade-offs between JSON and XML, explaining JSON's rise in web APIs.

- [Corbado Glossary: "What is CBOR?"](https://www.corbado.com/glossary/cbor)
    - A clear explanation of CBOR's purpose, benefits (efficiency, compactness), relationship to JSON, and relevance in the IoT context.

- [DuCharme, B. "A brief, opinionated history of XML"](https://www.bobdc.com/blog/a-brief-opinionated-history-of/)
    - Offers valuable historical context on XML's roots in SGML and its early development and adoption.
