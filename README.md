# Proof-of-concept macro for `#[cfg]` gating visibility

Disclaimer: while the syntax is an if-else-if-else chain,
the macro does not actually implement `else` semantics (yet).
Thus you need to do that manually. Adding that should not be
too difficult, and should be done before publishing.

I (@CAD97) am not really invested enough to actually do that.
I'd be willing to help someone else do that, though.
(Perhaps you can just reuse [`cfg_if!`](https://crates.io/crates/cfg-if)?)

## Example

This is [cfg-pub/examples/example.rs](cfg-pub/examples/example.rs).

```rust
use cfg_pub::cfg_pub;

#[cfg_pub(
    if #[cfg(SET_CFG)] pub
    else if #[cfg(not(SET_CFG))] pub(self)
)]
fn main() {
    println!("do something");
}
```

```powershell
PS> $env:RUSTFLAGS=$null
PS> cargo expand --example example
```

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use cfg_pub::cfg_pub;
#[cfg(not(SET_CFG))]
pub(self) fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["do something\n"],
            &match () {
                () => [],
            },
        ));
    };
}
```

```powershell
PS> $env:RUSTFLAGS="--cfg SET_CFG"
PS> cargo expand --example example
```

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use cfg_pub::cfg_pub;
#[cfg(SET_CFG)]
pub fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["do something\n"],
            &match () {
                () => [],
            },
        ));
    };
}
```
