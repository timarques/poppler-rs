# poppler-sys

Low level bindings to [poppler](https://gitlab.freedesktop.org/poppler/poppler).

## Features (TODO)

- glib-api
- qt5-api
- cpp-api

For now, only glib api is available.

## Bindings

Run `cargo run` on `poppler-sys` directory for binding generation.
(various modules are generated independently so this may take a while because of redundant generation)

Links to:
- `poppler` (dynamic)

Build-depends on:
- `pkg-config`
- `poppler`
- `cairo`
- `glib-2.0`

The bindings configuration tweaks assumes `poppler-0.77.0`,
but they may work fine on other versions.
