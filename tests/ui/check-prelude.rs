// #[path = "../utils/prelude.rs"]
// mod prelude;
//
// use prelude::*;
// use std::mem::size_of;

pub mod prelude {
    pub use crate::s::S;
    pub struct A;
}

mod s {
    pub struct S;
}

use prelude::*;

pub fn f() {
    let _a = A;
    let _s = s::S;
}
