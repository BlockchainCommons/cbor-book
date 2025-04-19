# From XML to JSON to CBOR

**NOTE:** You probably don't need to read this, unless you are interested in historical context. If you just want to know how to use CBOR, you can skip this section and go straight to [Using CBOR in Practice](../part_1/using_cbor_in_practice.md).

## A *Lingua Franca* for Data?

In the intricate dance of modern computing, data is the currency, and its seamless exchange is the lifeblood of countless applications. From the web pages we browse, to the complex interactions between microservices, to the faint signals transmitted by tiny sensors in the Internet of Things (IoT), the ability for disparate systems to represent, share, and understand structured information is fundamental. Yet, this universal need for communication hasn't always been met by a single, perfect language. Instead, the history of computing is marked by an evolution of data interchange formats, each rising to meet the specific challenges and technological landscapes of its era.

This narrative traces the lineage of three pivotal players in this evolution: the Extensible Markup Language ([XML](https://www.w3.org/TR/xml/)), JavaScript Object Notation ([JSON](https://www.json.org/)), and the Concise Binary Object Representation ([CBOR](https://cbor.io/)). We will explore their origins, delving into the motivations that sparked their creation. We will examine their core design principles, understanding the trade-offs inherent in their structures. Finally, we will follow their adoption trajectories, observing how they shaped, and were shaped by, the changing demands of the digital world. It's a journey that begins with the need for robust document structure, shifts towards simplicity and web-centric performance, and ventures into the realm of binary efficiency for the increasingly constrained devices connecting our world. Understanding this evolution reveals not just the technical specifications of these formats, but the underlying pressures and innovations that drive the quest for better ways to speak the language of data.

## The Age of Structure: XML's Rise from Publishing Roots

The story of modern data interchange formats arguably begins not with the web, but with the challenges of large-scale electronic publishing decades earlier. The foundation was laid by SGML, a powerful but complex standard, which XML would later refine and adapt for the burgeoning internet age.

### The SGML Inheritance: Laying the Foundation

In the 1960s and 1970s, as computerized typesetting gained traction, researchers at IBM—Charles Goldfarb, Ed Mosher, and Ray Lorie—grew frustrated with the proprietary nature of existing systems. They envisioned a nonproprietary, generic way to mark up documents, focusing on describing the _content_ and _structure_ rather than just the visual presentation. Their work led to the Generalized Markup Language (GML), which evolved into the Standard Generalized Markup Language (SGML). SGML was formally adopted as international standard ISO 8879 in 1986.

SGML's core idea was revolutionary: it provided rules for defining _other_ markup languages. Using SGML, one could create a specific vocabulary (a set of tags) and a grammar (a Document Type Definition, or DTD) to define the structure of a particular class of documents. This allowed for the creation of machine-readable documents designed for longevity, capable of remaining understandable for decades, independent of specific processing systems.

Due to its power and emphasis on structure and longevity, SGML found significant adoption in sectors dealing with complex, large-scale documentation, such as government, military (e.g., CALS DTD), aerospace, legal publishing, and heavy industry. However, SGML's specification was extensive and complex—over 150 technical pages—covering numerous special cases and features that made writing parsers difficult and limited its broader appeal.

A pivotal moment came with the advent of the World Wide Web. When Tim Berners-Lee needed a way to structure hypertext documents, SGML, being a text-based, flexible, non-proprietary standard, was a logical choice. HyperText Markup Language (HTML) was conceived as an SGML application, a relatively simple DTD defining tags for web documents. Dan Connolly created the first HTML DTD in 1992. While HTML rapidly became ubiquitous, its focus shifted towards presentation, and its evolution involved many browser-specific extensions, making it less suitable as a general-purpose data interchange format. SGML remained too complex for widespread web use, leaving a gap for a format that could bring SGML's structural power to the internet in a more manageable form.

### W3C and the Birth of XML: Taming SGML for the Web

By the mid-1990s, the need for a more structured way to exchange data over the web, beyond HTML's presentational focus, became apparent. In 1996, the World Wide Web Consortium (W3C) established an XML Working Group, initially known as the SGML Editorial Review Board, chaired by Jon Bosak of Sun Microsystems. Their mission was clear: create a simplified subset of SGML, often dubbed "SGML-lite," that would be suitable for internet delivery and processing, retaining SGML's core benefits of extensibility and structure while shedding much of its complexity.

The W3C XML Working Group operated with a clear set of design goals, formalized in the XML 1 Specification, which became a W3C Recommendation in February 1998. These goals shaped the language profoundly:

1. **Internet Usability**: XML should be straightforwardly usable over the internet.
2. **Broad Applicability**: It should support a wide variety of applications, not just web browsers.
3. **SGML Compatibility**: XML documents should be conforming SGML documents.
4. **Ease of Processing**: It should be easy to write programs that process XML documents.
5. **Minimal Optional Features**: Optional features should be kept to an absolute minimum, ideally zero.
6. **Human Readability**: XML documents should be human-legible and reasonably clear.
7. **Rapid Design**: The design process should be quick.
8. **Formal and Concise Design**: The specification should be formal and concise, amenable to standard parsing techniques.
9. **Ease of Creation**: XML documents should be easy to create, even with simple tools.
10. **Terseness is Minimally Important**: Unlike some SGML features designed to reduce typing, conciseness of markup was not a primary goal for XML.

The goal of SGML compatibility proved particularly insightful. It wasn't merely about adhering to a parent standard; it was a pragmatic decision with significant implications for adoption. SGML, despite its complexity, had an existing ecosystem of tools, expertise, and deployed systems. By defining XML as a valid subset of SGML, the working group ensured that existing SGML parsers and tools, such as specialized versions of Adobe FrameMaker 7, could process XML documents from day one of the standard's release in 1998. This dramatically lowered the barrier to entry for organizations already using SGML and provided an instant software ecosystem, accelerating XML's initial uptake. Furthermore, adhering to the SGML subset constrained the design choices, helping the working group achieve the goal of rapid development. This leveraging of a pre-existing, albeit complex, ecosystem demonstrated a powerful strategy for launching a new standard.

### Designing XML: Tags, Attributes, Namespaces, and Schemas

XML's fundamental structure is based on describing data using nested elements marked by tags. An element consists of a start tag (e.g., `<customer>`), an end tag (e.g., `</customer>`), and content in between, which can be text or other nested elements. Start tags can also contain attributes, which provide additional metadata about the element (e.g., `<address type="billing">`). XML also defines a specific syntax for empty elements, which have no content, such as `<br/>` or, alternatively, `<br></br>`. This tag-based structure makes the logical hierarchy of the data explicit and relatively easy for humans to read.

As XML usage grew, the need arose to combine elements from different "vocabularies" or schemas within a single document without causing naming collisions (e.g., distinguishing a `<table>` element for furniture from a `<table>` element for data). This led to the "Namespaces in XML" Recommendation in January 1999. Namespaces allow elements and attributes to be qualified by associating them with a unique Internationalized Resource Identifier (IRI), typically a URI. This is achieved using an `xmlns` attribute, often with a prefix (e.g., `xmlns:addr="http://www.example.com/addresses"`), allowing elements to be uniquely identified (e.g., `<addr:street>`). A default namespace can also be declared (`xmlns="URI"`), applying to unprefixed elements within its scope, but notably, default namespaces do not apply to attributes; attributes must be explicitly prefixed if they belong to a namespace other than that of their parent element. While URIs are used for uniqueness, they don't necessarily need to point to an actual online resource.

To enforce structure and validate the content of XML documents, schema definition languages are used. Initially, XML inherited Document Type Definitions (DTDs) from SGML. DTDs define the allowed elements, attributes, and their nesting rules. However, DTDs have limitations, such as using a non-XML syntax and lacking support for data types. To address this, the W3C developed XML Schema Definition ([XSD](https://www.w3.org/TR/xmlschema11-1/)), which became a Recommendation in 2001. XSD provides a much more powerful way to define XML structure, constrain content using a rich set of built-in and user-defined data types, and specify rules like cardinality and uniqueness. XSD schemas are themselves written in XML and use namespaces extensively.

XML's well-defined structure also enabled the development of powerful supporting technologies. XPath provides a language for navigating and selecting nodes within an XML document. XSL Transformations ([XSLT](https://www.w3.org/TR/xslt20/)) uses XPath to transform XML documents into other formats (like HTML or plain text). Programmatic access to XML documents is typically handled via APIs like the Document Object Model (DOM), which represents the entire document as a tree in memory, or the Simple API for XML (SAX), an event-based streaming API developed collaboratively outside the W3C to address early API incompatibilities.

This combination of extensibility (defining custom tags) and powerful schema validation (especially XSD) allowed XML to model complex, domain-specific data structures effectively. However, this power was a double-edged sword. Defining robust XSD schemas could be quite complex, leading some developers to seek simpler alternatives like RELAX NG or Schematron. Namespaces, while solving the collision problem, introduced another layer of complexity for both developers writing documents and programmers writing parsers. Furthermore, XML's flexibility sometimes meant that the same conceptual data could be represented in multiple valid ways, potentially hindering interoperability unless specific profiles or stricter conventions were agreed upon. This inherent complexity, coupled with XML's verbosity, eventually fueled the search for simpler alternatives, particularly in contexts where ease of use and performance were prioritized over rigorous validation and expressive power. The tension between richness and simplicity became a defining factor in the subsequent evolution of data formats.

### XML's Reign and Ripples: Adoption and Impact

Following its standardization in 1998, XML rapidly gained traction and became a dominant force in various computing domains throughout the early 2000s. Its promise of a standard, platform-independent way to structure and exchange data resonated widely.

One of the most significant areas of adoption was in **Web Services**. XML formed the bedrock of the SOAP (Simple Object Access Protocol) standard, which defined an XML-based messaging framework for exchanging structured information in distributed environments, often over HTTP. Accompanying technologies like WSDL (Web Services Description Language), which used XML Schema to describe the capabilities of a web service, and UDDI (Universal Description, Discovery and Integration) solidified the "WS-*" stack, enabling complex enterprise integrations.

XML also became a ubiquitous choice for **Configuration Files**. Many applications and frameworks adopted XML for storing settings due to its structured nature and human readability. Examples include configuration for Java logging frameworks like Log4j 39, Microsoft.NET application configuration (`web.config`, `app.config`) 36, build tools like Apache Ant 41, and numerous other system and application parameters.

Beyond configuration and services, XML fulfilled its original promise in **Document Formats and Publishing**. It became the basis for numerous structured document standards, including XHTML (an XML reformulation of HTML), RSS and Atom for content syndication, KML for geographic data, and specialized industry formats like DocBook for technical documentation. Its ability to separate content from presentation made it invaluable for multi-channel publishing (e.g., print and web from the same source) and content management systems.

More broadly, XML served as a general-purpose **Data Interchange** format, facilitating communication between diverse systems and applications. Its platform and vendor independence were seen as key advantages, helping to avoid vendor lock-in and simplifying data migration. Furthermore, its descriptive nature was considered beneficial for the long-term preservation of electronic materials.

This widespread adoption fostered a rich ecosystem. Numerous XML parsers (both commercial and open-source) became available. Tools for XML editing, validation, transformation (XSLT), and data binding (generating code from schemas) emerged. Conferences dedicated to XML and markup technologies flourished, building a strong community of practice.

### The Seeds of Change: XML's Verbosity Challenge

Despite its success, XML carried the seeds of its own partial decline in certain areas. A key factor was one of its original design goals: "Terseness in XML markup is of minimal importance". This principle, intended to simplify XML compared to SGML's complex minimization features and prioritize clarity, manifested in the requirement for explicit start and end tags for every element.

While this structure enhanced human readability and made parsing conceptually straightforward, it inevitably led to verbosity. Representing simple data structures often required significantly more characters in XML compared to potentially more compact formats. For example, a simple key-value pair like `{"name": "Alice"}` in JSON might become `<name>Alice</name>` in XML, adding substantial overhead, especially for large datasets with many small elements.

This verbosity, initially a minor concern in document-centric or enterprise integration scenarios (like SOAP), became a significant drawback as the web evolved. The rise of Asynchronous JavaScript and XML (AJAX) techniques in the mid-2000s emphasized frequent, small data exchanges between web browsers and servers to create more dynamic user interfaces. In this context, minimizing bandwidth usage and client-side parsing time became critical. XML's larger payloads and relatively complex parsing requirements (compared to potential alternatives) presented performance bottlenecks.

The XML community itself recognized the need for more efficient representations, leading to initiatives like the W3C's Efficient XML Interchange (EXI) Working Group, which developed a standardized binary XML format. While EXI offered significant compaction, it perhaps highlighted the inherent difficulty of retrofitting efficiency onto XML's text-based, tag-oriented foundation without adding another layer of complexity.

The explicit decision to deprioritize terseness, while crucial for distinguishing XML from SGML and achieving readability, had an unintended long-term consequence. As the web shifted towards dynamic applications and APIs prioritizing speed and efficiency, XML's defining characteristic of verbose, explicit structure became a liability. This created a fertile ground for a new format that would optimize for precisely what XML had considered of minimal importance: conciseness and ease of parsing, particularly within the context of web browsers and JavaScript.

## The Quest for Simplicity: JSON's Emergence in the Web 2.0 Era

As XML's verbosity and complexity began to chafe in the fast-paced world of web development, particularly with the rise of AJAX, the stage was set for a simpler, lighter alternative. That alternative emerged directly from the language powering the dynamic web: JavaScript.

### JavaScript's Offspring: Douglas Crockford and the "Discovery" of JSON

The story of JSON (JavaScript Object Notation) is inextricably linked with Douglas Crockford, an American computer programmer known for his work on JavaScript and his advocacy for its "good parts". In 2001, Crockford, along with Chip Morningstar and others, co-founded State Software. Their goal was ambitious for the time: to build stateful, multi-user, single-page web applications that communicated with a server in real-time, without relying on browser plugins like Flash or Java applets, which were common then.

They needed a lightweight format for data exchange between their Java server and the JavaScript running in the browser. Crockford realized that JavaScript's own object literal syntax—the way objects are defined directly in code (e.g., `{ key: value }`)—could serve this purpose. This data could be sent from the server embedded within a snippet of JavaScript, and the browser's native JavaScript engine could parse it, initially using the `eval()` function. Crockford often refers to this as a "discovery" rather than an invention, acknowledging that the idea was inherent in JavaScript and that others may have used similar techniques previously; he cites an instance at Netscape using JavaScript array literals for data communication as early as 1996.

The initial implementation involved sending an HTML document containing a `<script>` tag that called a JavaScript function on the parent page, passing the data as an object literal argument. A small refinement was needed: JavaScript reserved words (like `do`) couldn't be used as unquoted object keys. To avoid restricting key names, Crockford mandated that all keys in the format must be enclosed in double quotes, treating them as strings and thus bypassing the reserved word issue.

Crockford and his colleagues initially wanted to call the format "JSML" (JavaScript Markup Language), but the acronym was already taken by the JSpeech Markup Language. They settled on "JavaScript Object Notation," or JSON. Finding that potential clients were hesitant to use an unspecified format, Crockford formalized it. In 2002, he acquired the domain [json.org](https://json.org) and published the grammar and a reference parser. The simplicity of the format quickly resonated, and developers began submitting parsers for various other programming languages, demonstrating its potential beyond just JavaScript.

### Motivation: A Lightweight Alternative for a Faster Web

The primary driving force behind JSON was the need for a data interchange format that was significantly simpler and lighter than XML. Crockford aimed for minimalism, believing that "the less we have to agree on in order to inter-operate, the more likely we're going to be able to inter-operate well". He envisioned a standard simple enough to fit on the back of a business card.

This quest for simplicity was a direct reaction to the perceived verbosity and complexity of XML, which dominated data exchange at the time. When challenged that JSON was merely reinventing XML, Crockford famously retorted, "The good thing about reinventing the wheel is that you can get a round one".

The timing was perfect. The rise of AJAX techniques created a strong demand for a format optimized for frequent, small data transfers between servers and browsers. While "AJAX" stood for "Asynchronous JavaScript and XML," JSON quickly proved to be a better fit for many AJAX use cases. Its syntax maps almost directly to JavaScript objects and arrays, making it trivial to parse and use on the client-side with JavaScript. Its lightweight nature reduced bandwidth consumption and improved the performance of web applications striving for desktop-like responsiveness. JSON was designed with pragmatic usability for web developers as a top priority.

Interestingly, while JSON was born directly from JavaScript and initially targeted the browser environment, its success wasn't confined to that ecosystem. The very simplicity that made it ideal for JavaScript also made it remarkably easy to parse and generate in a multitude of other programming languages. The core data structures it represents—objects (or maps/dictionaries), arrays (or lists), strings, numbers, booleans, and _null_—are fundamental building blocks available in nearly all modern languages. Combined with its minimal and regular syntax, implementing JSON support was significantly less effort than building a full XML parser with DTD/Schema and namespace support. This ease of cross-language implementation was a major factor in its rapid adoption, transforming it from a JavaScript-specific solution into a de facto standard for web APIs and configuration files across the software industry. Simplicity, in this case, became a powerful catalyst for language independence and widespread adoption.

### Designing JSON: Key-Value Pairs, Arrays, and Minimal Types

JSON's syntax is intentionally minimal, built upon just a few structural elements derived from JavaScript. The primary structures are:

- **Objects**: Unordered collections of key-value pairs, enclosed in curly braces `{}`. Keys must be strings (always enclosed in double quotes), followed by a colon `:`, and then the value. Key-value pairs are separated by commas `,`. Example: `{ "name": "Alice", "age": 30 }`.
- **Arrays**: Ordered sequences of values, enclosed in square brackets ``. Values are separated by commas `,`. Example: `[ "apple", "banana", "cherry" ]`.

The values within objects and arrays can be one of a small set of fundamental data types 45:

- **String**: A sequence of Unicode characters, enclosed in double quotes `"`. Single quotes are not allowed.
- **Number**: A numeric value. JSON does not distinguish between integers and floating-point numbers in its specification, though implementations might. Scientific notation (e.g., `2.99E1`) is permitted.
- **Boolean**: The literal values `true` or `false` (lowercase).
- **Null**: The literal value `null` (lowercase).
- **Object**: A nested JSON object.
- **Array**: A nested JSON array.

This simple, text-based structure makes JSON highly human-readable. It directly represents common programming data structures (dictionaries/maps and lists/arrays), making it intuitive for developers.

Notably, JSON deliberately omits features found in XML, such as comments, namespaces, and attributes. The lack of comments was an explicit design decision by Crockford. He observed that comments in formats like XML were sometimes misused to embed parsing directives or metadata, which could break interoperability if parsers treated them differently. By disallowing comments entirely, JSON aimed to ensure that all parsers would interpret the data identically, prioritizing strict interoperability. If commentary is needed, the suggested approach is to include it as regular data within the JSON structure itself, perhaps using a conventional key name like `"_comment"`.

The integration of JSON with JavaScript was further solidified with the introduction of native `JSON.parse()` (to convert a JSON string into a JavaScript object/value) and `JSON.stringify()` (to convert a JavaScript object/value into a JSON string) methods in the ECMAScript 5 standard (published 2009). These built-in methods provided a safe and standardized way to handle JSON, replacing the potentially insecure use of `eval()` for parsing. The `stringify` method also offers optional `replacer` functions or whitelists to control or transform the output, and objects can define a `toJSON()` method to customize their serialization.

### JSON vs. XML: A Paradigm Shift

Comparing JSON and XML reveals fundamental differences in philosophy and design, reflecting the different problems they were initially designed to solve.

- **Format Type**: XML is a _markup language_ derived from SGML, designed to describe documents with structure and semantics. JSON is strictly a _data interchange format_, derived from JavaScript object literals.
- **Structure**: XML uses a hierarchical, tag-based _tree structure_ with elements, attributes, and text content. JSON uses _key-value pairs_ (like maps or dictionaries) and ordered _arrays_ (lists).
- **Verbosity & Size**: XML is inherently more verbose due to start/end tags, potentially attributes, and namespaces. JSON's minimal syntax results in more compact representations for equivalent data, often leading to significantly smaller file sizes (e.g., 30-40% smaller in some comparisons 38).
- **Readability**: Both are text-based and human-readable, but JSON's simpler structure is often considered cleaner and easier to grasp quickly.
- **Parsing**: JSON parsing is generally simpler and faster, leveraging native capabilities in JavaScript and straightforward libraries in other languages. XML requires dedicated parsers that handle tags, attributes, namespaces, and potentially schema validation, making parsing more complex and computationally intensive.
- **Features & Extensibility**: XML offers built-in support for comments, namespaces for managing vocabularies, attributes for metadata on elements, and powerful schema languages (DTD, XSD) for strict validation and data typing. JSON is intentionally minimal, lacking these features in its core specification. Extensibility and validation in the JSON world are typically handled by separate specifications like [JSON Schema](https://json-schema.org/) or conventions built on top of the basic format (like [JSON-LD](https://json-ld.org/) for linked data).
- **Data Types**: JSON supports a limited set of fundamental types (string, number, boolean, null, object, array). XML itself doesn't enforce types without a schema, but XSD allows for a very rich set of data types, including dates, binary (via encoding), and custom types.

This comparison highlights a paradigm shift: XML prioritized structure, extensibility, and validation, suitable for complex documents and enterprise systems, while JSON prioritized simplicity, ease of use, and performance, making it ideal for the fast-moving world of web APIs and client-side scripting.

### Rapid Ascent: JSON Becomes the Language of APIs

JSON's simplicity and alignment with web technologies led to its rapid and widespread adoption, particularly from the mid-2000s onwards, coinciding with the "Web 2.0" era and the popularization of AJAX. It quickly became the dominant format for **RESTful web APIs**, displacing XML in many cases. Industry surveys indicated that a vast majority (over 85% in one report) of APIs were using JSON as their default format.

Its utility extended beyond APIs. JSON found use in **configuration files** for applications and tools, and as a **data storage** format, notably within NoSQL databases like MongoDB (which uses a binary variant, [BSON](https://bsonspec.org/)) and for client-side storage using browser APIs like `localStorage`.

The growth was largely organic, fueled by developer adoption and the ease with which parsers could be created for different languages, as evidenced by the numerous implementations submitted to [json.org](https://json.org). Formal standardization followed this adoption, with JSON being specified in [ECMA-404](https://ecma-international.org/publications-and-standards/standards/ecma-404/) and later as [IETF RFC 8259](https://datatracker.ietf.org/doc/html/rfc8259).

A crucial factor contributing to JSON's enduring success and widespread adoption is its remarkable stability. Crockford has emphasized that JSON is essentially "finished"—there is no version number, and the core specification has remained unchanged since its inception. This stability contrasts sharply with technologies that undergo frequent revisions, which can lead to fragmentation and compatibility issues (a problem CBOR later explicitly aimed to avoid by designing for extensibility without version negotiation). By providing a simple, reliable, and predictable foundation, JSON allowed a rich ecosystem of tools, libraries, and dependent specifications (like JSON Schema and JSON-LD) to flourish around it, rather than constantly adapting to changes within it. In the realm of foundational infrastructure technologies, this kind of stability proved to be a powerful, perhaps even decisive, feature.

## The Need for Speed (and Size): CBOR Enters for Efficiency

While JSON offered a much-needed simplification and performance boost over XML for web APIs, its text-based nature still presented limitations in certain demanding environments. The relentless push for greater efficiency, particularly driven by the rise of the Internet of Things (IoT), paved the way for a format that combined JSON's data model with the compactness and speed of binary encoding: CBOR.

### Beyond Text: The Motivation for Binary

Text-based formats like JSON, despite their readability, have inherent inefficiencies compared to binary representations:

- **Parsing Speed**: Parsing text involves interpreting character sequences, which can be computationally more expensive than decoding structured binary data. Binary formats can often map more directly to machine data types.
- **Message Size**: Representing numbers, booleans, and especially repeated keys as text strings consumes more bytes than optimized binary encodings. Binary formats can use fixed-size representations or variable-length encodings that are typically more compact. Studies comparing CBOR to JSON often show significant size reductions for the same data.
- **Binary Data Handling**: JSON lacks a native type for arbitrary binary data (like images, cryptographic keys, raw sensor readings). Such data must be encoded into strings, usually using Base64, which adds complexity and increases data size by roughly 33%.

These limitations become particularly acute in **constrained environments**, a defining characteristic of the Internet of Things (IoT). IoT devices often operate under severe constraints:

- **Limited Resources**: Low processing power (CPU), minimal memory (RAM and storage), and restricted energy budgets (battery power).
- **Constrained Networks**: Low bandwidth, high latency, and potentially unreliable wireless communication links (e.g., [LoRaWAN](https://lora-alliance.org/), [NB-IoT](https://www.gsma.com/solutions-and-impact/technologies/internet-of-things/narrow-band-internet-of-things-nb-iot/), [Bluetooth LE](https://www.bluetooth.com/)).

In such scenarios, minimizing message size is crucial for conserving bandwidth and energy, while minimizing processing overhead (parsing/serialization) is vital for performance and battery life. CBOR (Concise Binary Object Representation) was explicitly designed to address these needs, aiming to provide the flexibility and schema-free nature of JSON's data model but in a much more compact and efficiently processable binary form.

### IETF Standardization: Building on the JSON Model

CBOR was developed within the Internet Engineering Task Force (IETF), the primary standards body for internet protocols, specifically to meet the needs of constrained environments. Key figures in its development include Carsten Bormann and Paul Hoffman.

Crucially, CBOR was not designed in a vacuum. It explicitly builds upon the successful and widely understood **JSON data model**. It supports equivalent types for JSON's numbers, strings (specifically UTF-8 text strings), arrays, maps (JSON's objects), booleans (`true`, `false`), and `null`. The major addition to the core model is a native type for **binary byte strings**, directly addressing a key limitation of JSON.

The standardization process resulted in RFC 7049, published in 2013. This was later obsoleted by [RFC 8949](https://datatracker.ietf.org/doc/html/rfc8949) in 2020, which became Internet Standard 94 (STD 94). Importantly, RFC 8949 provides clarifications, editorial improvements, and incorporates fixes based on implementation experience, but it _maintains full wire-format compatibility_ with RFC 7049; it does not define a new version of CBOR.

The design goals articulated in the RFCs clearly position CBOR relative to its predecessors:

- **Compact Code Size**: Encoders and decoders must be implementable with very small code footprints, suitable for memory-constrained devices (e.g., IETF [RFC 7228](https://tools.ietf.org/html/rfc7228) Class 1 nodes). This was prioritized over absolute message compactness.
- **Reasonable Message Size**: The serialization should be reasonably compact, significantly smaller than JSON, but without resorting to complex compression schemes that would violate the code size goal.
- **Extensibility without Version Negotiation**: The format must be extensible for future needs, but in a way that allows older decoders to process messages containing extensions they don't understand. This avoids the versioning pitfalls that can plague evolving protocols.
- **Schema-Free Decoding**: Like JSON, CBOR data items should be self-describing, allowing generic decoders to parse the structure without needing a predefined schema.
- **Broad Applicability**: Suitable for both constrained nodes and high-volume applications due to frugal CPU usage.
- **JSON Compatibility**: Support all JSON data types and allow reasonable conversion to/from JSON where data models overlap.

CBOR's design reflects a conscious effort to learn from the successes and failures of both JSON and XML. It embraces JSON's simple and familiar data model, making it relatively easy for developers already comfortable with JSON to understand and potentially interoperate with. It directly tackles JSON's shortcomings for constrained environments by introducing native binary support and optimizing for size and processing efficiency. Furthermore, unlike core JSON but reminiscent of XML's capabilities, CBOR incorporates a formal mechanism for semantic extensibility through **tags**. This allows new data types (like dates, bignums, URIs, or application-specific structures) to be layered onto the base format in a standardized way, registered via [IANA](https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml), without requiring changes to the core specification or breaking backward compatibility for basic decoders. This synthesis demonstrates a mature approach to standards design: leveraging familiarity, addressing specific weaknesses, providing controlled extensibility, and optimizing for the target environment's constraints.

### CBOR's Niche: Adoption in Constrained Environments

Given its design goals emphasizing compactness and efficiency, CBOR has found its primary adoption foothold in the **Internet of Things (IoT)** and other **constrained environments**. Its ability to represent complex data structures (inherited from the JSON model) in a binary format with low overhead makes it well-suited for devices and networks where resources are scarce.

Several IETF protocols and frameworks designed for constrained environments leverage CBOR:

- [CoAP (Constrained Application Protocol)](https://coap.space/): A lightweight application-layer protocol analogous to HTTP but designed for constrained networks, often uses CBOR as its payload format. Mappings exist for protocols like IEC 61850 (used in smart grids) over CoAP+CBOR, demonstrating performance benefits over HTTP/XML or WS-SOAP alternatives in constrained settings.
- [COSE (CBOR Object Signing and Encryption)](https://datatracker.ietf.org/doc/rfc8152/): Defines standard ways to represent cryptographic keys, signatures, MACs, and encrypted messages using CBOR. COSE builds on the concepts of [JOSE (JSON Object Signing and Encryption)](https://datatracker.ietf.org/doc/html/rfc7519) but benefits from CBOR's native binary handling and compactness. COSE is a fundamental building block for security in many IoT protocols and is notably used in the FIDO Alliance's WebAuthn standard for passkey authentication.
- [ACE (Authentication and Authorization for Constrained Environments)](https://datatracker.ietf.org/doc/html/rfc9200): A framework defining protocols for securing access to resources in constrained IoT environments, often utilizing CBOR and COSE.
- **Device Management**: Protocols like [CORECONF](https://core-wg.github.io/comi/draft-ietf-core-comi.html) leverage CBOR for model-driven management of constrained devices, applying concepts similar to NETCONF/YANG but adapted for the IoT.
- **Certificate Representation**: The [C509](https://datatracker.ietf.org/doc/draft-ietf-cose-cbor-encoded-cert/) specification defines a CBOR encoding for X.509 public key certificates, aiming to reduce the size overhead compared to traditional DER/ASN.1 encoding for use in constrained devices.

Beyond IETF standards, CBOR is being explored for compacting semantic web data formats like JSON-LD for use in the Web of Things (WoT), with proposed schemes like [CBOR-LD](https://json-ld.github.io/cbor-ld-spec/) and [CBL](https://arxiv.org/abs/2407.04398) aiming for significant size reductions.

Supporting this adoption is the availability of [CBOR implementations](https://cbor.io/impls.html) across a wide range of programming languages, including C, C++ (with libraries like libcbor, TinyCBOR, jsoncons), Go, Rust, Python (cbor2), Java, Swift, Lua, Haskell, OCaml, and more. This broad language support facilitates the integration of CBOR into diverse systems.

However, it's important to note that while CBOR's adoption is significant and growing within its target niche of constrained systems and security protocols, it is still relatively young compared to the decades-long history of XML and the widespread dominance of JSON in general web APIs. Its focus on binary efficiency comes at the cost of direct human readability, making it less suitable for use cases where easy inspection and manual editing of the data format itself are primary requirements.

### The Trajectory: CBOR's Place and Future

CBOR represents a specialized evolution in data formats, consciously optimizing for binary efficiency and compactness while retaining the flexible data model popularized by JSON. Its trajectory appears firmly rooted in environments where these optimizations provide tangible benefits, primarily the resource-constrained world of IoT, M2M communication, and security-sensitive protocols.

Its future growth seems likely to continue along these lines. As billions more IoT devices come online, the need for efficient communication protocols and data formats will only intensify, strengthening CBOR's relevance. The integration of CBOR into fundamental security mechanisms like COSE, particularly with the rise of passwordless authentication (WebAuthn/Passkeys), provides another strong vector for adoption. Furthermore, its built-in extensibility mechanism via semantic tags offers a pathway for adapting to future requirements and new data types without breaking the core format or requiring disruptive versioning.

In Part II of this book, we'll also extensively explore another benefit of a native binary format like CBOR: determinism, which is critical for cryptographic applications. The deterministic encoding of CBOR data structures allows for consistent serialization and deserialization, which is essential for cryptographic signatures and hashing. This property is particularly valuable in contexts where data integrity and authenticity are paramount, such as in secure messaging, digital signatures, and distributed consensus protocols.

That said, CBOR is unlikely to displace JSON in the broader landscape of web APIs and general data interchange. JSON's human readability, vast existing infrastructure, and native integration with JavaScript remain compelling advantages in contexts where binary efficiency and cryptographic determinism are not overriding concerns.

## Conclusion: An Evolving Landscape of Data Representation

The evolution from XML to JSON to CBOR illustrates a recurring pattern in technology: a move from general-purpose, feature-rich solutions towards more specialized formats optimized for specific constraints and use cases. SGML was highly general but complex. XML simplified SGML for web and document structure. JSON further simplified things, optimizing for the specific needs of web APIs (simplicity, performance). CBOR then specialized again, optimizing for the binary efficiency demanded by constrained environments. While general formats like XML and JSON remain valuable, specialized formats like CBOR often achieve superior performance within their intended niche by making different trade-offs (e.g., sacrificing human readability for size and speed). The future of data interchange is likely one of coexistence, where the choice of format depends critically on the specific requirements of the application and its environment, rather than a single format "winning" across the board.

**Comparative Overview of XML, JSON, and CBOR**

|   |   |   |   |
|---|---|---|---|
|**Feature**|**XML**|**JSON**|**CBOR**|
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
