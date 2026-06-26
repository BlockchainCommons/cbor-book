# justfile for cbor-book

# Prevent accidental use of implicit shell mode
set shell := ["bash", "-cu"]

# Display this help message by default
default:
    @just --summary

# 🚀 Deploy book to GitHub Pages (idempotent)
deploy:
    ./deploy

# Open the GitHub Pages URL
read:
    open https://blockchaincommons.github.io/cbor-book/

# 🔧 Build book locally
build:
    mdbook build

# 👀 Serve book locally with live reload
serve:
    mdbook serve --open

# 📖 Build the book as an EPUB (output in book/epub/)
epub:
    mdbook build

# 🧼 Clean deploy worktree and build artifacts
clean:
    git worktree remove --force /tmp/book-deploy || true
    rm -rf /tmp/book-deploy
    rm -rf book

# 🧪 Run Rust code tests
test:
    cargo test

# 📂 Open built book in browser (after build)
open:
    open book/index.html
