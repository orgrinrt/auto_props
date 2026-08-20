auto_props
============
[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/auto_props.svg)](https://github.com/orgrinrt/auto_props/stargazers) 
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/auto_props.svg)](https://github.com/orgrinrt/auto_props/issues) 
[![Current Version](https://img.shields.io/badge/version-0.1.0-green.svg)](https://github.com/orgrinrt/auto_props) 

A macro for implementing properties, optionally with builder-style methods, for the lazy.

---
## Buy me a coffee

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

---

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

---

## License
>You can check out the full license [here](https://github.com/orgrinrt/auto_props/blob/main/LICENSE)

This project is licensed under the terms of the **MPL-2.0** license.
