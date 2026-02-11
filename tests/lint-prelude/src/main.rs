fn main() {
    // `klint::not_using_prelude` is not triggered.
    use a::a;
    use prelude::*;

    // warning: this item is available via prelude
    //  --> src/main.rs:2:5
    //   |
    // 2 |     use src::a::a;
    //   |     ^^^^^^^^^^^^^^
    //   |
    //   = help: import with `dep::prelude::*` instead
    //   = note: `#[warn(klint::not_using_prelude)]` on by default
    // use dep::a::a;
    // use dep::prelude::*;

    a();
    b();
}

pub mod a {
    pub fn a() {}
}

pub mod prelude {
    pub use super::a::a;
    pub fn b() {}
}
