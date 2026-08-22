# `auto_props`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/auto_props.svg)](https://github.com/orgrinrt/auto_props/stargazers)
[![Crates.io](https://img.shields.io/crates/v/auto_props)](https://crates.io/crates/auto_props)
[![docs.rs](https://img.shields.io/docsrs/auto_props)](https://docs.rs/auto_props)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/auto_props.svg)](https://github.com/orgrinrt/auto_props/issues)
![License](https://img.shields.io/github/license/orgrinrt/auto_props?color=%23009689)

> A macro for implementing properties, optionally with builder-style methods, for the lazy.

</div>

## Installation

Not published yet, so this does not resolve. It is the command once a release
lands.

```bash
cargo add auto_props
```

Or in `Cargo.toml`:

```toml
[dependencies]
auto_props = "0.1"
```

## Usage

`property!` writes the accessor surface into a trait. The getter and setter are declarations an
implementor fills in; the builder-style `with_*` arrives with a body already, and that is the part
the macro saves you writing.

```rust
use auto_props::property;

trait Named {
    property!(name: String);
}

struct Thing {
    name: String,
}

impl Named for Thing {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn set_name(&mut self, value: String) {
        self.name = value;
    }
}

let mut thing = Thing { name: "before".into() };
thing.set_name("after".into());
assert_eq!(thing.name(), "after");

// with_name is generated whole, not declared
let built = Thing { name: "before".into() }.with_name("built".into());
assert_eq!(built.name(), "built");
```

Both assertions were run against the crate rather than written from the macro's name.

The trade is narrow enough to state plainly. You still write the getter and the setter, because only
the implementor knows how the value is stored. What you stop writing is the consuming `with_*` for
every field on every type, which is the boilerplate that actually accumulates.

## Features

Two, and both change the surface the macro declares, so the example above is written against the
defaults.

| Feature | Default | What it changes |
|---|---|---|
| `impl_with` | on | Emits the consuming `with_*` builder method with a body. Turning it off leaves you the getter and setter only. |
| `getter_prefix` | off | Names the getter `get_name` rather than `name`. Turning it on means the implementation in the example above no longer satisfies the trait. |

`getter_prefix` is worth being deliberate about: it is not additive. Enabling it renames a method
every implementor already writes, so it is a choice made once for a crate rather than per call site.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/auto_props/blob/dev/LICENSE)
