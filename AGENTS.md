# The CBOR, dCBOR, and Gordian Envelope Book - Developer Documentation

## Rust Toolchain

`mdbook` requires the Rust toolchain to be installed. If you don't have it installed, follow the steps at the following link:

- <https://www.rust-lang.org/tools/install>

## `mdbook` Installation

To build and view the documentation, you need to have `mdbook` and the other necessary dependencies installed. Follow these steps:

1. Install `mdbook` and the necessary plugins:

```bash
cargo install mdbook --version 0.4.52 \
  && cargo install mdbook-admonish --version 1.20.0 \
  && cargo install mdbook-embed --version 0.2.0 \
  && cargo install mdbook-variables --version 0.3.0 \
  && cargo install mdbook-linkcheck --version 0.7.7 \
  && cargo install mdbook-pagetoc --version 0.2.0 \
  && cargo install mdbook-inline-highlighting --version 1.0.0 \
  && cargo install mdbook-epub --version 0.4.51
```

The pinned versions are needed in order to build specifically for ePub. These are the newest versions of the plugins compatible with `mdbook ^0.4` and able to build the book without issues.

## CBOR, dCBOR, and Envelope Syntax Highlighting

To enable build-time syntax highlighting for CBOR, dCBOR, and Envelope code blocks in the documentation, you need to set up the `mdbook-dcbor-shiki` preprocessor. Follow these steps:

2. Build the `mdbook-dcbor-shiki` preprocessor:

```bash
cd cbor-book/mdbook-dcbor-shiki
cargo install --path .
```

3. Install the required Node.js dependencies:

```bash
cd cbor-book
npm install
```

## Building and Serving the Documentation

4. Serve the documentation locally:

```bash
cd cbor-book
mdbook serve --open
```

This will open the documentation in your default web browser, and live update as you make changes to the Markdown files.

## Deploy changes

To deploy changes to the documentation, you can use the following command:

```bash
cd cbor-book
./deploy
```

This script will build the documentation and push the changes to the `gh-pages` branch of the repository, making it available at [cborbook.com](https://cborbook.com). It may take a few minutes for the changes to propagate.

## Documentation Structure

`mdbook` uses the `src/SUMMARY.md` file to organize the content into chapters and sections. Each section corresponds to a Markdown file in the `src/` directory.

The documentation is organized into the following sections:

- **Overview**: Introduction, resources, and recommended reading.
- **User Guide**: Comprehensive instructions on using Flying Logic.
- **Scripting Guide**: Details on automating tasks using Flying Logic's scripting capabilities.
- **Thinking with Flying Logic**: Guides on applying the Theory of Constraints and other methodologies.

The source files for the documentation are located in the `src/` directory, and the generated HTML files are in the `book/` directory.

## Building the Documentation

To build the documentation as a static site, run:

```bash
mdbook build
```

The output will be available in the `book/` directory. This directory is *not* tracked by Git, as it is generated from the source files.

## Tools and Plugins

The documentation uses the following tools and plugins:

- [`mdbook`](https://rust-lang.github.io/mdBook): The main tool for building and serving the documentation.
- [`mdbook-admonish`](https://tommilligan.github.io/mdbook-admonish): For creating admonition blocks (notes, warnings, etc.).
- [`mdbook-embed`](https://github.com/kumavale/mdbook-embed): For embedding external content like videos.
- [`mdbook-variables`](https://crates.io/crates/mdbook-variables): For interpolating variables like the copyright year.
- [`mdbook-linkcheck`](https://github.com/Michael-F-Bryan/mdbook-linkcheck): For checking the validity of image links in the documentation.
- [`mdbook-pagetoc`](https://github.com/slowsage/mdbook-pagetoc): For generating a table of contents for each page.
- [`mdbook-inline-highlighting`](https://crates.io/crates/mdbook-inline-highlighting): For inline syntax highlighting within text.
- [`mdbook-epub`](https://crates.io/crates/mdbook-epub): For building the book as an EPUB (`book/epub/`).

And the custom preprocessor:

- `mdbook-dcbor-shiki` For syntax highlighting of CBOR, dCBOR, and Envelope code blocks.
