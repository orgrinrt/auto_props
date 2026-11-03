# `auto_props`

> A macro for implementing properties on a trait, with builder-style methods generated for you.

`property!` writes the accessor surface into a trait. The getter and setter are declarations the implementor fills in, because only the implementor knows how the value is stored, and the consuming `with_*` builder method arrives with a body already, which is the part that otherwise gets written for every field on every type.

## Usage

Not published to crates.io yet, so it is a git dependency for now.

```toml
[dependencies]
auto_props = { git = "https://github.com/orgrinrt/auto_props", branch = "dev" }
```

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

## Features

`impl_with` is on by default and emits the `with_*` method with a body, and without it only the getter and setter are declared. `getter_prefix` is off by default and names the getter `get_name` and not `name`, so enabling it renames a method every implementor already writes, which makes it a choice for a whole crate and not a call site. The example above is written against the defaults.

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/auto_props/blob/dev/LICENSE)
