# Zero to Production, written in Rust

I'm only about half way through the book.

## Docker Build Size (Multistage vs Standard)

| Build                     | Size  |
|---------------------------|-------|
| williamnoble/zero2prod:v1 | 3.2 GiB |
| williamnoble/zero2prod:v2 | 34.3 MiB  |

Further reduction via cross-compilation (alpine linux) using rust-musl-builder.

