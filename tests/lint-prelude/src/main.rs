fn main() {
    use src::a::a;
    use src::prelude::*;

    a();
    b();
}

// warning: this item is available via prelude
//  --> src/main.rs:2:5
//   |
// 2 |     use src::a::a;
//   |     ^^^^^^^^^^^^^^
//   |
//   = help: import with `dep::prelude::*` instead
//   = note: `#[warn(klint::not_using_prelude)]` on by default
use dep as src;

// warning: unused import: `super::a::a`
//   --> src/main.rs:18:17
//    |
// 18 |         pub use super::a::a;
//    |                 ^^^^^^^^^^^
//    |
//    = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default
// use module as src;
// mod module {
//     pub mod a {
//         pub fn a() {}
//     }
//
//     pub mod prelude {
//         pub use super::a::a;
//         pub fn b() {}
//     }
// }
